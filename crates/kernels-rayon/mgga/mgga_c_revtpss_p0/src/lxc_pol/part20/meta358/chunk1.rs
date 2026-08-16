//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1303/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1303(t2710: f64, t2793: f64, t39494: f64, t2804: f64, t874: f64, t9288: f64, t10535: f64, t231: f64, t2645: f64, t281: f64, t68: f64, t211: f64, t9644: f64) -> (f64, f64, f64, f64) {
    let t39633 = 0.20561456923286030469e-1_f64 * t2710 * t2793 * t39494;
    let t39635 = t874 * t2804 * t9288;
    let t39640 = t10535 * t281 * t68 * t2645 * t231;
    let t39643 = 1.0_f64 / t9644 / t211;
    (t39633, t39635, t39640, t39643)
}
