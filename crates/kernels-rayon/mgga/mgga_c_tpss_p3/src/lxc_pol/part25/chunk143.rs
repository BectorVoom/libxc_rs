//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 143/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk143(t425: f64, t434: f64, t294: f64, t410: f64, t412: f64, t421: f64, t332: f64, t56: f64, t390: f64) -> (f64, f64, f64, f64) {
    let t435 = t425 * t434;
    let t438 = t294 * (-0.310907e-1_f64 * t412 * t421 + t410 - 0.19751673498613801407e-1_f64 * t435);
    let t440 = 0.19751673498613801407e-1_f64 * t294 * t435;
    let t441 = t56 * t332;
    let t442 = 1.0_f64 / t390;
    (t438, t440, t441, t442)
}
