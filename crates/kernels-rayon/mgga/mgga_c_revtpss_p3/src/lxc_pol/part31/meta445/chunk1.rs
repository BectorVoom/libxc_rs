//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1591/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1591(t141: f64, t20273: f64, t6461: f64, t698: f64, t6464: f64, t6467: f64, t6422: f64, t689: f64) -> (f64, f64, f64, f64, f64) {
    let t20274 = t141 * t20273;
    let t20276 = t698 * t6461;
    let t20278 = t698 * t6464;
    let t20280 = t698 * t6467;
    let t20283 = t689 * t6422;
    (t20274, t20276, t20278, t20280, t20283)
}
