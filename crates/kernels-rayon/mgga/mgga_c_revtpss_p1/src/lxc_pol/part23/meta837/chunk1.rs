//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2709/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2709(t21102: f64, t3704: f64, t21094: f64, t3172: f64, t5384: f64, t17361: f64, t5274: f64, t5261: f64, t5390: f64, t12915: f64, t20703: f64, t247: f64) -> (f64, f64, f64, f64, f64) {
    let t69674 = t21102 * t3704;
    let t69698 = t5384 * t3172 * t21094;
    let t69700 = t5274 * t17361;
    let t69710 = t5261 * t5390;
    let t69719 = t5384 * t247 * t12915 * t20703;
    (t69674, t69698, t69700, t69710, t69719)
}
