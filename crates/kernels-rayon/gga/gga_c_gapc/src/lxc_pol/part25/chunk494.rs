//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 494/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk494(t2315: f64, t2801: f64, t330: f64, t197: f64, t617: f64, t953: f64, t1793: f64, t889: f64, t2233: f64, t2636: f64, t1686: f64, t325: f64) -> (f64, f64, f64, f64, f64) {
    let t2802 = t2801 * t2315;
    let t2803 = t330 * t2802;
    let t2804 = t197 * t2803;
    let t2807 = t617 * t953;
    let t2810 = t889 * t1793;
    let t2811 = t2636 * t2233;
    let t2814 = t325 * t1686;
    (t2804, t2807, t2810, t2811, t2814)
}
