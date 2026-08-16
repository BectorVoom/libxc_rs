//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 753/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk753(t5296: f64, t618: f64, t1769: f64, t1780: f64, t1776: f64, t144: f64, t174: f64, t46: f64, t5181: f64, t616: f64, t1692: f64, t2660: f64, t51: f64, t568: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5297 = t5296 * t618;
    let t5299 = t1769 * t1780;
    let t5301 = t1769 * t1776;
    let t5304 = 1.0_f64 / t174 / t144;
    let t5305 = t5304 * t46;
    let t5307 = t5305 * t616 * t5181;
    let t5312 = t2660 * t51 * t568 * t1692;
    (t5297, t5299, t5301, t5304, t5305, t5307, t5312)
}
