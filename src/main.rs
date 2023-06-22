use std::{io, time, thread};

fn main() {
println!("      

                                              *
                                            *  /*
                                            **\\ *
                                            \\**
                                            *)
                                           *                         
                                       ****   .*****                            
                                            *,,,,,%%%%%%%%%                     
                        %%%%%%%%%%%%%%%%     ,,,,,%%%%%%%%%%%                   
                     .%%,.%%%%%%%%%%%%%%     ,,,,,%%%%%%%%%%%%%                 
                   %%%......%%%%%%%%%%%%%*   ,,,,,%%%%%%%%%%%%%%%               
                %%%...........%%%%%%%%%%%%%% ,,%%%%%%%%%%%%%%%%%%%%             
             %%%................%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%           
          %%%.....................%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%        
       %%%   ......................%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%      
             ........................%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%       
             ..........................%%%%%%%%%%%%%%%%%%%%%%%%%%%#             
             ...........................*%%%%%%%%%%%%%%%%%%%*******             
              .......#..................***%%%%%%%%%%**************             
              .......(((((((((..........****,%%********************             
           @%/.......(((((((((..........***************************/%@          
      @/%&%#%%.......(((((((((..........***************************%%#%&%/@     
   @/%%#%#%%%%.......,((((((((..........***************************%%%%%%##%/@  
  @%%%%%%%%%%%........((((((((..........***************************%%%%#%%#%#%@ 
 @%%%#%%%%%%%%........((((((((..........*************************%%%%%%%%%##%%%@
 /&###%%%%%%%%%% .....((((((((..........*******************%%%%%%%%%%%%%%%%%%%%(
 /%%%%%%%%%%%%%%%%%%%%%% (((((..........*************%%%%%%%%%%%%%%%%%%%%%#%%%%@
 @((%#%%%%%%%%%%%%%%%%%%%%%%%%%%........*******%%%%%%%%%%%%%%%%%%%%%%%%%%#%##&/@
  @/&%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%#%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%/@ 
    @/%#%%#%#%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%#%%#%/@   
       @@/%&%%%#%%#%%%#%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%#%%%%%%%#%%%%%#&%/@@      
              @@/#%%&(#%%##%%%##%%%%%#######%%%%%##%%%##%%#(&%%(/@@             


                Welcome to Matthew's roof calculator!!!
    ");


    let ten_millis = time::Duration::from_millis(100);
    let now = time::Instant::now();
    
    thread::sleep(ten_millis);
    println!("Enter desired length from chimney to roof:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input = input.trim_end_matches(&['\r', '\n'][..]).to_owned();
    let mut a:f64 = input.parse().expect("error: invalid length");

    println!("Enter chimey width:");
     input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input = input.trim_end_matches(&['\r', '\n'][..]).to_owned();
    let mut width: f64 = input.parse().expect("error: invalid width");
    input = String::new();
    println!("Enter roof angle in degrees:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input = input.trim_end_matches(&['\r', '\n'][..]).to_owned();
    let mut angle: f64 = input.parse().expect("error: invalid angle");

    
    let mut b = 0.0;
    let mut c: f64 = 0.0;
    let mut scaler = 1.0;

    let a_1_miter = 90.0;
    let mut a_2_miter = 0.0;
    let mut b_1_miter = 0.0;
    let mut b_2_miter = 0.0;
    let mut c_1_miter = 0.0;
    let mut c_2_miter = 0.0;
    let mut c_1_bevel = 0.0;
    let mut b_1_bevel = 0.0;
    angle = angle.to_radians();
    //while b < 18.0 {
        a *= scaler;
    //let a_2 = 90.0 - angle;
    let ratio = angle.tan();
    let opposite = ratio * a; //may not work this is assuming 12 for A works

    c = ((width / 2.0) * (width / 2.0) + opposite* opposite).sqrt();
    c_1_miter = 90.0 - (opposite / (width / 2.0)).to_degrees();
    c_2_miter = 90.0 - ((width / 2.0) / opposite).to_degrees();
    let hyp = opposite / angle.sin();
    b = (hyp * hyp + (width / 2.0) * (width / 2.0)).sqrt();
    a_2_miter = 90.0 - angle.to_degrees();

    b_2_miter = 90.0 - ((width/ 2.0) / hyp).to_degrees();
    b_1_miter = 90.0 - (hyp / (width/ 2.0)).to_degrees();

    b_1_bevel = 90.0 - angle.to_degrees();
    c_1_bevel = 90.0 - angle.to_degrees();

    c -=  2.25 * c_1_miter.to_radians().tan();
    c -=  2.25 * c_2_miter.to_radians().tan();
    b -=  2.25 * b_2_miter.to_radians().tan();
    //b -= 2.25;
    //a -= 2.25 * a_2_miter.to_radians().tan();

    // b += 0.75 * b_1_bevel.to_radians().tan();
    // c += 0.75 * c_1_bevel.to_radians().tan();
    //     scaler = (18.0 / b) * (b / a);
    // }

    //calculate added length for each piece


    
    println!("length of each 2x6:");
    println!("A length: {:?} inches",a);
    println!("A2 miter:{:?} degrees",a_2_miter);
    println!("");
    println!("B length: {:?} inches",b);
    println!("B1 miter:{:?} degrees",b_1_miter);
    println!("B2 miter:{:?} degrees",b_2_miter);
    println!("B1 bevel:{:?} degrees",b_1_bevel);
    println!("");
    println!("C length: {:?} inches",c);
    println!("C1 miter:{:?} degrees",c_1_miter);
    println!("C2 miter:{:?} degrees",c_2_miter);
    println!("C1 bevel:{:?} degrees",c_1_bevel);
    println!("");
    
    


    
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read input");



}
