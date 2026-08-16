//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 840/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk840(t11669: f64, t2660: f64, t2696: f64, t277: f64, t2977: f64, t775: f64, t761: f64, t778: f64, t13: f64, t2: f64, t3151: f64, t3157: f64, t721: f64) -> (f64, f64, f64, f64, f64) {
    let t11778 = 0.19263893255070628431e1_f64 * t11669 * t2696 * t2660;
    let t11780 = 480.0_f64 * t2977 * t277;
    let t11784 = t775 * t775;
    let t11787 = t761 * t761;
    let t11788 = t778 * t778;
    let t11792 = 0.24955700379505800916e5_f64 * t13 / t11784 * t11787 / t11788;
    let t11795 = t3157 * t2 * t3151 * t721;
    (t11778, t11780, t11787, t11792, t11795)
}
