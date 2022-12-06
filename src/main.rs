fn main() { 
  let  x = 4;
  println!("x is:{}",x);
  let  x = x + 5;
  {
    let x = 2;
    println!("x is: {}", x);
  }
  let x = x +1;

  println!("x is:{}",x);

  let x = "helo";
  println!("x is: {}", x);

}

