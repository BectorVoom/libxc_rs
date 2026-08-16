//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2001/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2001(t10510: f64, t25399: f64, t10115: f64, t1951: f64, t7058: f64, t92871: f64, t1032: f64, t11007: f64, t233: f64, t25372: f64, t10509: f64, t25377: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93273 = t25399 * t10510;
    let t93276 = 0.11044544084478153697e-3_f64 * t10115 * t1951;
    let t93278 = 0.22487184191643109717e-1_f64 * t7058 * t92871;
    let t93279 = t1032 * t11007;
    let t93280 = t93279 * t233;
    let t93281 = t25372 * t93280;
    let t93285 = t25377 * t10509;
    (t93273, t93276, t93278, t93280, t93281, t93285)
}
