//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3272/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3272<F: Float>(t18353: F, t2689: F, t18394: F, t2703: F, t10777: F, t14686: F, t61715: F, t837: F, t14872: F, t14894: F, t18426: F, t18444: F, t2646: F, t2745: F, t2747: F, t2749: F, t40284: F, t40836: F, t4364: F, t51042: F, t51047: F, t61791: F) -> F {
    let t62129 = t2689 * t18353;
    let t62135 = t2703 * t18394;
    let t62148 = t10777 * t14686 * t61715 * t837;
    let t62158 = -F::cast_from(0.91476005056713590803e-4_f64) * t51042 - F::cast_from(0.2032800112371413129e-4_f64) * t40836 + F::cast_from(0.15244095330869239812e-2_f64) * t62129 - F::cast_from(0.12862205435420921092e-2_f64) * t14894 * t4364 * t18426 * t40284 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t62135 - F::cast_from(0.80031500487063509015e-2_f64) * t51047 - F::cast_from(0.42874018118069736972e-3_f64) * t2745 * t4364 * t61791 * t837 - F::cast_from(0.21437009059034868486e-3_f64) * t2745 * t4364 * t18444 * t2646 - F::cast_from(0.25410001404642664112e-4_f64) * t62148 + F::cast_from(0.17149607247227894789e-2_f64) * t2745 * t2747 * t61791 * t2749 + F::cast_from(0.85748036236139473944e-3_f64) * t2745 * t2747 * t18444 * t14872;
    t62158
}
