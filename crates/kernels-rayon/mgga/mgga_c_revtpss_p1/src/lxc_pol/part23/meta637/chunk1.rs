//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2340/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2340(t14545: f64, t251: f64, t786: f64, t4503: f64, t860: f64, t10115: f64, t883: f64, t2710: f64, t2793: f64, t39494: f64, t2804: f64, t874: f64, t9288: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39597 = t14545 * t251;
    let t39598 = t786 * t39597;
    let t39608 = t4503 * t860;
    let t39609 = t786 * t39608;
    let t39624 = t10115 * t883;
    let t39633 = 0.20561456923286030469e-1_f64 * t2710 * t2793 * t39494;
    let t39635 = t874 * t2804 * t9288;
    (t39597, t39598, t39609, t39624, t39633, t39635)
}
