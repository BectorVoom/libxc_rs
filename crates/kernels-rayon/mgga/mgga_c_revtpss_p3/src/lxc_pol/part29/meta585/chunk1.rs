//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1938/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1938(t28108: f64, t644: f64, t77: f64, t2315: f64, t7705: f64, t1497: f64, t6977: f64, t1927: f64, t4241: f64, t7719: f64, t13272: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t101200 = t77 * t28108 * t644;
    let t101204 = t77 * t7705 * t2315;
    let t101214 = t6977 * t1497;
    let t101218 = t1927 * t4241;
    let t101226 = t7719 * t644;
    let t101230 = t13272 * t607;
    (t101200, t101204, t101214, t101218, t101226, t101230)
}
