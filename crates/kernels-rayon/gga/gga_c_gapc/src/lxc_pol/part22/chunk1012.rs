//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1012/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1012(t11683: f64, t6943: f64, t11682: f64, t3737: f64, t6948: f64, t6951: f64, t640: f64, t919: f64, t3243: f64, t128: f64, t329: f64, t2536: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11684 = t11683 * t6943;
    let t11685 = t11682 * t11684;
    let t11687 = t3737 * t6948;
    let t11688 = t11683 * t6951;
    let t11689 = t11687 * t11688;
    let t11691 = t640 * t919;
    let t11692 = t3243 * t11691;
    let t11694 = t128 * t329;
    let t11695 = t11694 * t2536;
    (t11684, t11685, t11687, t11688, t11689, t11691, t11692, t11694, t11695)
}
