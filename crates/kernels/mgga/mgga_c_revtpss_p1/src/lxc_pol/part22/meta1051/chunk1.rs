//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3706/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3706<F: Float>(t70158: F, t70172: F, t70186: F, t70200: F, t1208: F, t21332: F, t225: F, t480: F, t1235: F, t1238: F, t1250: F, t16720: F, t17254: F, t17693: F, t20945: F, t21275: F, t247: F, t371: F, t3719: F, t372: F, t44291: F, t44293: F, t482: F, t57021: F, t57026: F, t57029: F, t57520: F, t70120: F, t70129: F, t70133: F, t70140: F) -> (F, F, F, F) {
    let t70202 = t70158 + t70172 + t70186 + t70200;
    let t70208 = t21332 * t1208;
    let t70209 = t70208 * t225;
    let t70210 = t70209 * t480;
    let t70213 = F::cast_from(0.51448821741683684368e-2_f64) * t57520 * t247 * t3719 * t70120 + F::cast_from(0.17149607247227894789e-2_f64) * t21275 * t17254 - F::cast_from(0.17149607247227894789e-2_f64) * t70129 - F::cast_from(0.52930886565518193793e-4_f64) * t70133 + F::cast_from(0.28582678745379824648e-2_f64) * t17693 * t20945 * t1250 * t16720 + F::cast_from(0.57165357490759649296e-3_f64) * t70140 + F::cast_from(0.1270341277572436651e-3_f64) * t44291 - F::cast_from(0.47637797908966374413e-4_f64) * t44293 + F::cast_from(0.11433071498151929859e-2_f64) * t57021 - F::cast_from(0.19055119163586549765e-3_f64) * t57026 - F::cast_from(0.3811023832717309953e-3_f64) * t57029 - F::cast_from(0.21437009059034868486e-3_f64) * t1235 * t371 * t372 * t482 * t70202 - F::cast_from(0.42874018118069736972e-3_f64) * t70210 * t1238;
    (t70202, t70208, t70209, t70213)
}
