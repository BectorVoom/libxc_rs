//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2025/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2025(t233: f64, t93279: f64, t25372: f64, t10996: f64, t25377: f64, t10509: f64, t25375: f64, t25296: f64, t25365: f64, t1957: f64, t2718: f64, t25386: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t93280 = t93279 * t233;
    let t93281 = t25372 * t93280;
    let t93282 = t25377 * t10996;
    let t93283 = t93281 * t93282;
    let t93285 = t25377 * t10509;
    let t93286 = t25375 * t93285;
    let t93297 = t25365 * t25296;
    let t93301 = t1957 * t2718;
    let t93302 = t25386 * t93301;
    (t93280, t93281, t93282, t93283, t93285, t93286, t93297, t93301, t93302)
}
