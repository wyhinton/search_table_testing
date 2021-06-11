
use fltk::{prelude::*,  table::*, button::*, draw::*, enums::*};
use crate::{CustomEvent};
mod table_button;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::convert::TryInto;

pub struct SearchTable{
    table: Table,
    items: Rc<RefCell<Vec<String>>>,
    buttons: Vec<Button>
}

impl SearchTable{
    pub fn new(x: i32, y: i32, w: i32, h: i32, items: Rc<RefCell<Vec<String>>>) -> SearchTable {
        //table setup
        let mut table = Table::new(x,y,w+20,h, None);
        table.end();
        table.set_rows(10);
        table.set_cols(5);
        table.set_row_height_all(90);
        
        

        //provide access to our strings inside of our table draw call
        let items_cl_1 = items.clone();
        table.draw(move|widg|{
            //provide access to our strings inside of draw_cell
            // let ic_cl_2 = items_cl_1.clone();
            draw_rect_fill(
                widg.x(),
                widg.y(),
                widg.width(),
                widg.height(),
                Color::Red,
            );
            //draw the children after drawing the background fill color
            widg.draw_children();
        });
        //draw our widgets again if we input into our search bar
        table.handle(move |widg, ev| 
          if ev == CustomEvent::SEARCH_INPUT.into(){
            // fltk::prelude::GroupExt::clear(widg); 
              widg.redraw();
            true
        } else {
            false
        });
        let mut buttons = vec![];
        for x in 0..100{
            buttons.push(Button::new(0,0,50,50, None));
        }
        let st = SearchTable{
            table: table,
            items: items.clone(),
            buttons: buttons,
        };

        st
    }
    //update our available list of strings
    pub fn set_values(&mut self, new_items: Vec<String>){
        *self.items.borrow_mut() = new_items;
        let ic_cl_2 = self.items.clone();
        self.table.draw_cell(move |t, ctx, row, col, x, y, w, h| {
            if let TableContext::Cell = ctx {
                let t_index = row*t.cols() as i32+ col;
                //create a new button intance 
                //maybe our buttons could be cached so we don't have to call the constructor over and over
                if t_index < *&ic_cl_2.borrow().len() as i32{
                    let mut button = Button::new(x, y, w, h,None);
                    button.set_label(&ic_cl_2.borrow()[t_index as usize] );
                    t.add(&button);
                }
                dbg!(t.children());
            }
        });
    }

}

impl Deref for SearchTable {
    type Target = Table;

    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

impl DerefMut for SearchTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.table
    }
}
