//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3548/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3548(t366: f64, t64907: f64, t19773: f64, t3215: f64, t11922: f64, t16067: f64, t19721: f64, t19566: f64, t3090: f64, t1025: f64, t1028: f64, t1045: f64, t15158: f64, t15691: f64, t15700: f64, t3097: f64, t3115: f64, t3117: f64, t3220: f64, t371: f64, t372: f64, t373: f64, t43121: f64, t4910: f64, t55265: f64, t55272: f64, t55279: f64, t55290: f64, t6273: f64, t64989: f64, t66395: f64) -> f64 {
    let t67516 = t64907 * t366;
    let t67521 = t19773 * t3215;
    let t67526 = t16067 * t11922 * t19721;
    let t67528 = t19566 * t3090;
    let t67543 = -0.21437009059034868486e-3_f64 * t1025 * t371 * t372 * t373 * t64989 - 0.42874018118069736972e-3_f64 * t67516 * t1028 - 0.21437009059034868486e-3_f64 * t19773 * t3220 - 0.28582678745379824648e-3_f64 * t67521 + 0.57165357490759649296e-3_f64 * t55265 - 0.1270341277572436651e-3_f64 * t55272 + 0.28582678745379824648e-3_f64 * t67526 + 0.28582678745379824648e-3_f64 * t67528 * t3097 - 0.19055119163586549765e-3_f64 * t55279 + 0.17149607247227894789e-2_f64 * t15700 * t15691 * t1045 * t15158 + 0.45732285992607719436e-2_f64 * t43121 * t6273 - 0.42874018118069736972e-3_f64 * t3115 * t3117 * t66395 * t4910 + 0.30488190661738479624e-2_f64 * t55290;
    t67543
}
