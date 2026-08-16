//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 345/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk345(t501: f64, t997: f64, t1016: f64, t605: f64, t1012: f64, t1628: f64, t1589: f64, t993: f64, t1007: f64, t2754: f64, t600: f64, t568: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2798 = t997 * t501;
    let t2801 = t1016 * t605;
    let t2804 = t1628 * t1012;
    let t2807 = t1589 * t993;
    let t2810 = t1628 * t1007;
    let t2815 = t600 * t2754;
    let t2816 = t568 * t2815;
    (t2798, t2801, t2804, t2807, t2810, t2816)
}
