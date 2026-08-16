//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3272/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3272(t18353: f64, t2689: f64, t18394: f64, t2703: f64, t10777: f64, t14686: f64, t61715: f64, t837: f64, t14872: f64, t14894: f64, t18426: f64, t18444: f64, t2646: f64, t2745: f64, t2747: f64, t2749: f64, t40284: f64, t40836: f64, t4364: f64, t51042: f64, t51047: f64, t61791: f64) -> f64 {
    let t62129 = t2689 * t18353;
    let t62135 = t2703 * t18394;
    let t62148 = t10777 * t14686 * t61715 * t837;
    let t62158 = -0.91476005056713590803e-4_f64 * t51042 - 0.2032800112371413129e-4_f64 * t40836 + 0.15244095330869239812e-2_f64 * t62129 - 0.12862205435420921092e-2_f64 * t14894 * t4364 * t18426 * t40284 + 7.0_f64 / 72.0_f64 * t62135 - 0.80031500487063509015e-2_f64 * t51047 - 0.42874018118069736972e-3_f64 * t2745 * t4364 * t61791 * t837 - 0.21437009059034868486e-3_f64 * t2745 * t4364 * t18444 * t2646 - 0.25410001404642664112e-4_f64 * t62148 + 0.17149607247227894789e-2_f64 * t2745 * t2747 * t61791 * t2749 + 0.85748036236139473944e-3_f64 * t2745 * t2747 * t18444 * t14872;
    t62158
}
