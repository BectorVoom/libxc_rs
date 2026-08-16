//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1839/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1839(t251: f64, t281: f64, t93238: f64, t1032: f64, t11007: f64, t233: f64, t25372: f64, t1957: f64, t2718: f64, t25386: f64, t786: f64, t860: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93240 = t281 * t93238 * t251;
    let t93279 = t1032 * t11007;
    let t93280 = t93279 * t233;
    let t93281 = t25372 * t93280;
    let t93301 = t1957 * t2718;
    let t93302 = t25386 * t93301;
    let t93314 = t25372 * t93301;
    let t93317 = t25386 * t93280;
    let t93320 = t786 * t860;
    (t93240, t93281, t93302, t93314, t93317, t93320)
}
