//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2093/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2093(t1497: f64, t6977: f64, t1926: f64, t1927: f64, t4241: f64, t25163: f64, t7715: f64, t644: f64, t7719: f64, t13272: f64, t607: f64, t10301: f64, t1470: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t101214 = t6977 * t1497;
    let t101215 = t1926 * t101214;
    let t101218 = t1927 * t4241;
    let t101219 = t1926 * t101218;
    let t101222 = t7715 * t25163;
    let t101226 = t7719 * t644;
    let t101227 = t1926 * t101226;
    let t101230 = t13272 * t607;
    let t101237 = t10301 * t1470;
    (t101215, t101219, t101222, t101227, t101230, t101237)
}
