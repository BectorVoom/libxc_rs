//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1019/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1019(t22626: f64, t539: f64, t6525: f64, t1860: f64, t1993: f64, t601: f64, t1864: f64, t1867: f64, t22075: f64, t592: f64, t6326: f64, t6322: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22627 = 384.0_f64 * t22626;
    let t22635 = t539 * t6525;
    let t22636 = 16.0_f64 * t22635;
    let t22641 = 0.21053604230838734656e2_f64 * t601 * t1993 * t1860;
    let t22652 = 0.51947267698127589897e2_f64 * t601 * t1864 * t22075 * t1867;
    let t22655 = 480.0_f64 * t6326 * t592;
    let t22656 = t6322 * t592;
    (t22627, t22636, t22641, t22652, t22655, t22656)
}
