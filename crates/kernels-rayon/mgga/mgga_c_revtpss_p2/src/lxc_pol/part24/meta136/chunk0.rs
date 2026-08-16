//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 718/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk718(t5571: f64, t762: f64, t1468: f64, t3874: f64, t1711: f64, t3881: f64, t1892: f64, t212: f64, t1358: f64, t689: f64, t1893: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5572 = t5571 * t762;
    let t5574 = t3874 * t1468;
    let t5582 = t3881 * t1711;
    let t5599 = t212 * t1892;
    let t5600 = t5599 * t1358;
    let t5601 = t689 * t5600;
    let t5603 = t786 * t1893;
    (t5572, t5574, t5582, t5599, t5600, t5601, t5603)
}
