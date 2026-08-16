//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 905/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk905(t17975: f64, t709: f64, t3780: f64, t3817: f64, t39: f64, t695: f64, t224: f64, t5266: f64, t13464: f64, t13468: f64, t13469: f64, t13474: f64, t13475: f64, t1417: f64, t1701: f64, t17933: f64, t17937: f64, t17941: f64, t17945: f64, t17946: f64, t17950: f64, t17958: f64, t17960: f64, t17964: f64, t17966: f64, t17971: f64, t2035: f64, t2387: f64, t3786: f64, t6757: f64) -> f64 {
    let t17976 = t17975 * t709;
    let t17980 = t3780 * t3817;
    let t17986 = t695 * t39;
    let t17987 = t224 * t17986;
    let t17988 = t5266 * t709;
    let t17992 = -0.23254900946437792e-2_f64 * t2387 * t17933 + 0.23254900946437792e-1_f64 * t2387 * t17937 + 0.23254900946437792e-1_f64 * t2387 * t17941 + 0.77462893625097599762e-3_f64 * t17945 * t13469 * t17946 - 0.38731446812548799881e-3_f64 * t13468 * t13469 * t17950 - 0.46509801892875584e-2_f64 * t13474 * t13475 * t17950 - 0.46509801892875584e-1_f64 * t17958 * t6757 * t17960 + 0.46509801892875584e-1_f64 * t17964 * t6757 * t17966 + 0.93019603785751168e-2_f64 * t17971 * t13475 * t17946 + 0.37540077436335915588e-1_f64 * t1417 * t1701 * t17976 - 0.11854761295685025975e-1_f64 * t1417 * t1701 * t17980 + 0.46509801892875584e-1_f64 * t13464 * t3786 - 0.14053536537767171586e-3_f64 * t17987 * t2035 * t17988;
    t17992
}
