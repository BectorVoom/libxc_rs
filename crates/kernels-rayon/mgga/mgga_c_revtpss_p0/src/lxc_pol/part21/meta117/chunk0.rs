//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 757/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk757(t2798: f64, t2801: f64, t72: f64, t860: f64, t686: f64, t874: f64, t2470: f64, t875: f64, t251: f64, t2718: f64) -> (f64, f64, f64, f64, f64) {
    let t2802 = t2798 * t2801;
    let t2804 = t860 * t72;
    let t2806 = t874 * t2804 * t686;
    let t2810 = 0.13009920719177044025e-1_f64 * t874 * t875 * t2470;
    let t2811 = t2718 * t251;
    (t2802, t2804, t2806, t2810, t2811)
}
