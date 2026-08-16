//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1889/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1889(t25374: f64, t99403: f64, t33: f64, t41154: f64, t1711: f64, t2411: f64, t1497: f64, t6977: f64, t1927: f64, t4241: f64, t644: f64, t7719: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99466 = t99403 * t25374;
    let t100981 = t41154 * t33;
    let t100987 = t2411 * t1711;
    let t101214 = t6977 * t1497;
    let t101218 = t1927 * t4241;
    let t101226 = t7719 * t644;
    (t99466, t100981, t100987, t101214, t101218, t101226)
}
