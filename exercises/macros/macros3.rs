// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a hint.

// macro_use属性有两种用途。首先，它可以通过作用于模块的方式让模块内的宏的作用域在模块关闭时不结束
// -- Rust 参考手册，3 宏 -- 声明宏
#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
