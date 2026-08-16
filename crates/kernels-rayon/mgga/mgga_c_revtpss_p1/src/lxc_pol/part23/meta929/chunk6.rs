//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3039/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3039(t1100: f64, t1102: f64, t198: f64, t336: f64, t5023: f64, t78094: f64, t78096: f64, t78099: f64, t78154: f64, t78478: f64, t78686: f64, t78690: f64, t78694: f64, t78696: f64, t78698: f64, t80166: f64, t80211: f64, t80819: f64, t80869: f64, t80918: f64, t80967: f64, t81015: f64, t81068: f64) -> f64 {
    let t81075 = -t5023 * t78478 * t1100 + t78094 + t78096 + t78099 - t78154 + t198 * t336 * (t80166 + t80211 + t80819 + t80869 + t80918 + t80967 + t81015 + t81068) * t1102 + t78686 + t78690 - t78694 - t78696 + t78698;
    t81075
}
