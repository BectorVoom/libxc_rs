//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 830/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk830(t7878: f64, t898: f64, t893: f64, t2586: f64, t2649: f64, t2612: f64, t309: f64, t300: f64, t2583: f64, t2587: f64, t3608: f64, t7359: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7879 = t7878 * t898;
    let t7880 = t893 * t7879;
    let t7882 = t2586 * t2649;
    let t7883 = t893 * t7882;
    let t7885 = t2612 * t309;
    let t7886 = t300 * t7885;
    let t7889 = t2583 * t2587;
    let t7891 = t3608 * t7359;
    (t7879, t7880, t7882, t7883, t7885, t7886, t7889, t7891)
}
