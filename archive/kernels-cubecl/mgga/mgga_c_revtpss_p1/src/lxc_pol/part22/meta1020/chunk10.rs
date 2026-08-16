//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3548/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3548<F: Float>(t366: F, t64907: F, t19773: F, t3215: F, t11922: F, t16067: F, t19721: F, t19566: F, t3090: F, t1025: F, t1028: F, t1045: F, t15158: F, t15691: F, t15700: F, t3097: F, t3115: F, t3117: F, t3220: F, t371: F, t372: F, t373: F, t43121: F, t4910: F, t55265: F, t55272: F, t55279: F, t55290: F, t6273: F, t64989: F, t66395: F) -> F {
    let t67516 = t64907 * t366;
    let t67521 = t19773 * t3215;
    let t67526 = t16067 * t11922 * t19721;
    let t67528 = t19566 * t3090;
    let t67543 = -F::cast_from(0.21437009059034868486e-3_f64) * t1025 * t371 * t372 * t373 * t64989 - F::cast_from(0.42874018118069736972e-3_f64) * t67516 * t1028 - F::cast_from(0.21437009059034868486e-3_f64) * t19773 * t3220 - F::cast_from(0.28582678745379824648e-3_f64) * t67521 + F::cast_from(0.57165357490759649296e-3_f64) * t55265 - F::cast_from(0.1270341277572436651e-3_f64) * t55272 + F::cast_from(0.28582678745379824648e-3_f64) * t67526 + F::cast_from(0.28582678745379824648e-3_f64) * t67528 * t3097 - F::cast_from(0.19055119163586549765e-3_f64) * t55279 + F::cast_from(0.17149607247227894789e-2_f64) * t15700 * t15691 * t1045 * t15158 + F::cast_from(0.45732285992607719436e-2_f64) * t43121 * t6273 - F::cast_from(0.42874018118069736972e-3_f64) * t3115 * t3117 * t66395 * t4910 + F::cast_from(0.30488190661738479624e-2_f64) * t55290;
    t67543
}
