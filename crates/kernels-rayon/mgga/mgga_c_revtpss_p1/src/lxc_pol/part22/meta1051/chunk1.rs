//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3706/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3706(t70158: f64, t70172: f64, t70186: f64, t70200: f64, t1208: f64, t21332: f64, t225: f64, t480: f64, t1235: f64, t1238: f64, t1250: f64, t16720: f64, t17254: f64, t17693: f64, t20945: f64, t21275: f64, t247: f64, t371: f64, t3719: f64, t372: f64, t44291: f64, t44293: f64, t482: f64, t57021: f64, t57026: f64, t57029: f64, t57520: f64, t70120: f64, t70129: f64, t70133: f64, t70140: f64) -> (f64, f64, f64, f64) {
    let t70202 = t70158 + t70172 + t70186 + t70200;
    let t70208 = t21332 * t1208;
    let t70209 = t70208 * t225;
    let t70210 = t70209 * t480;
    let t70213 = 0.51448821741683684368e-2_f64 * t57520 * t247 * t3719 * t70120 + 0.17149607247227894789e-2_f64 * t21275 * t17254 - 0.17149607247227894789e-2_f64 * t70129 - 0.52930886565518193793e-4_f64 * t70133 + 0.28582678745379824648e-2_f64 * t17693 * t20945 * t1250 * t16720 + 0.57165357490759649296e-3_f64 * t70140 + 0.1270341277572436651e-3_f64 * t44291 - 0.47637797908966374413e-4_f64 * t44293 + 0.11433071498151929859e-2_f64 * t57021 - 0.19055119163586549765e-3_f64 * t57026 - 0.3811023832717309953e-3_f64 * t57029 - 0.21437009059034868486e-3_f64 * t1235 * t371 * t372 * t482 * t70202 - 0.42874018118069736972e-3_f64 * t70210 * t1238;
    (t70202, t70208, t70209, t70213)
}
