//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1182/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1182(t24503: f64, t7495: f64, t2246: f64, t2661: f64, t2667: f64, t2730: f64, t312: f64, t508: f64, t2670: f64, t2678: f64, t2679: f64, t2668: f64, t2674: f64) -> (f64, f64, f64, f64, f64) {
    let t24504 = t24503 * t7495;
    let t24507 = t2661 * t2246 * t2667;
    let t24510 = t2730 * t2667;
    let t24513 = t508 * t312;
    let t24514 = t24513 * t2670;
    let t24516 = t2678 * t24514 * t2679;
    let t24519 = t2668 * t24514 * t2674;
    (t24504, t24507, t24510, t24516, t24519)
}
