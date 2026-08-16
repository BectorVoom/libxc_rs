//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1181/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1181(t10959: f64, t3835: f64, t7359: f64, t7433: f64, t875: f64, t10888: f64, t2678: f64, t2669: f64, t7835: f64, t10: f64, t2666: f64, t2662: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24492 = t3835 * t10959 * t7359;
    let t24494 = t7433 * t875;
    let t24496 = t2678 * t24494 * t10888;
    let t24498 = t2669 * t7835;
    let t24502 = t2666 * t10;
    let t24503 = t2662 * t24502;
    (t24492, t24494, t24496, t24498, t24502, t24503)
}
