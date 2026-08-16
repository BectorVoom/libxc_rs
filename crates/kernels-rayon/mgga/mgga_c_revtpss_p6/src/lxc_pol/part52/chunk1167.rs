//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1167/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1167(t1882: f64, t561: f64, t125625: f64, t247: f64, t31752: f64, t5675: f64, t121116: f64, t33926: f64, t121011: f64, t1399: f64, t1426: f64, t1903: f64) -> (f64, f64, f64, f64) {
    let t125627 = t561 * t1882;
    let t125630 = t31752 * t125625 * t247 * t125627 * t5675;
    let t125632 = t121116 * t33926;
    let t125637 = t121011 * t247 * t1426 * t1903 * t1399;
    (t125627, t125630, t125632, t125637)
}
