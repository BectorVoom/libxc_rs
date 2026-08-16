//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2085/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2085(t33: f64, t41154: f64, t1711: f64, t2411: f64, t1497: f64, t6977: f64, t1927: f64, t4241: f64, t644: f64, t7719: f64, t13272: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t100981 = t41154 * t33;
    let t100987 = t2411 * t1711;
    let t101214 = t6977 * t1497;
    let t101218 = t1927 * t4241;
    let t101226 = t7719 * t644;
    let t101230 = t13272 * t607;
    (t100981, t100987, t101214, t101218, t101226, t101230)
}
