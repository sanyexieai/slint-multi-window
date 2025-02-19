use slint::SharedString;
use std::rc::Rc;

// 导入由 slint-build 生成的界面代码
slint::slint!{
    import { MainWindow } from "ui/main_window.slint";
    import {  SubWindow} from "ui/sub_window.slint";
    export { MainWindow,SubWindow }
}

fn main() {
    let main_window = MainWindow::new().unwrap();
    
    main_window.on_open_sub_window(move || {
        let sub = SubWindow::new().unwrap();
        sub.set_message(SharedString::from("这是一个新的子窗口！"));
        sub.show().unwrap();
    });

    main_window.run().unwrap();
} 