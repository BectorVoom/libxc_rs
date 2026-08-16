//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3504/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3504(t1058: f64, t19858: f64, t15688: f64, t16509: f64, t1053: f64, t11632: f64, t11703: f64, t15604: f64, t15691: f64, t1592: f64, t15973: f64, t16226: f64, t16230: f64, t19450: f64, t19857: f64, t225: f64, t3117: f64, t3133: f64, t3151: f64, t3155: f64, t366: f64, t375: f64, t42690: f64, t4899: f64, t53320: f64, t53322: f64, t53332: f64, t53741: f64, t53805: f64, t53810: f64, t53820: f64, t6092: f64, t60927: f64, t65057: f64) -> f64 {
    let t66093 = t19858 * t1058;
    let t66114 = t16509 * t15688;
    let t66127 = -0.3811023832717309953e-3_f64 * t53805 + 0.21437009059034868486e-3_f64 * t65057 * t225 * t366 * t375 + 0.28582678745379824648e-3_f64 * t66093 - 0.22866142996303859718e-2_f64 * t19857 * t1053 * t375 - t53320 * t53332 * t60927 / 9.0_f64 + 7.0_f64 / 162.0_f64 * t53320 * t53322 * t60927 - 0.42874018118069736972e-3_f64 * t42690 * t3117 * t19450 * t15604 - 0.23818898954483187207e-3_f64 * t4899 * t11703 * t6092 * t15973 + 0.57165357490759649296e-3_f64 * t53810 + 0.76220476654346199061e-3_f64 * t53820 + 0.11433071498151929859e-2_f64 * t66114 * t16230 + 0.57165357490759649296e-3_f64 * t16226 * t15691 * t3155 * t1592 * t3133 + 0.17149607247227894789e-2_f64 * t53741 * t15691 * t11632 * t1592 * t3151;
    t66127
}
