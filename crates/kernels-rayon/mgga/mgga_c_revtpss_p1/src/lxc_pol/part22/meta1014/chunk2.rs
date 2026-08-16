//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3493/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3493(t20050: f64, t3188: f64, t20054: f64, t1063: f64, t18946: f64, t247: f64, t3109: f64, t11714: f64, t11991: f64, t20046: f64, t3106: f64, t42257: f64, t42270: f64, t42274: f64, t53542: f64, t53557: f64, t53559: f64, t6323: f64, t6327: f64, t6331: f64) -> f64 {
    let t65801 = t3188 * t20050;
    let t65803 = t3188 * t20054;
    let t65807 = t1063 * t247 * t3109 * t18946;
    let t65819 = -t42257 / 972.0_f64 - 0.5081365110289746604e-3_f64 * t42270 - 0.1270341277572436651e-3_f64 * t42274 - t53542 / 324.0_f64 + 0.31758531939310916276e-3_f64 * t65801 + 0.19055119163586549765e-3_f64 * t65803 + 0.19055119163586549765e-3_f64 * t65807 - 0.2540682555144873302e-2_f64 * t11714 * t6327 - 0.15244095330869239812e-2_f64 * t11714 * t6323 - 0.15244095330869239812e-2_f64 * t3106 * t20046 - 0.28582678745379824648e-3_f64 * t11991 * t6331 + 0.17149607247227894789e-2_f64 * t53557 + 0.3811023832717309953e-3_f64 * t53559;
    t65819
}
