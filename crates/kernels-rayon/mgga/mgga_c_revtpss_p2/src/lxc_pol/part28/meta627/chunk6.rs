//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2253/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2253(t644: f64, t7719: f64, t1926: f64, t13272: f64, t607: f64, t2248: f64, t77: f64, t7705: f64, t10301: f64, t1470: f64, t2247: f64, t4181: f64) -> (f64, f64, f64, f64, f64) {
    let t101226 = t7719 * t644;
    let t101227 = t1926 * t101226;
    let t101230 = t13272 * t607;
    let t101234 = t77 * t7705 * t2248;
    let t101237 = t10301 * t1470;
    let t101240 = t2247 * t4181;
    (t101227, t101230, t101234, t101237, t101240)
}
