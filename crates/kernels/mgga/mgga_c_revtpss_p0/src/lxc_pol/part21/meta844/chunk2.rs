//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3159/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3159<F: Float>(t1179: F, t16831: F, t1744: F, t3477: F, t3520: F, t5155: F, t12552: F, t1749: F, t1161: F, t1169: F, t1189: F, t12418: F, t12473: F, t12504: F, t12548: F, t12556: F, t17086: F, t17089: F, t1745: F, t3447: F, t3516: F, t3524: F, t45181: F, t5143: F, t5158: F, t57808: F, t57814: F, t57816: F, t57820: F, t58005: F, t58023: F, t58053: F, t58116: F, t58129: F, t58149: F, t58177: F, t58200: F, t58227: F) -> F {
    let t58234 = t16831 * t1179;
    let t58237 = t3477 * t1744;
    let t58242 = t5155 * t3520;
    let t58247 = t1749 * t12552;
    let t58250 = t57808 + t57814 - t57816 + F::cast_from(0.2069040516770936012e4_f64) * t58005 * t12473 + F::new(1.0) * t45181 * t1745 + F::new(3.0) * t12418 * t5143 + F::new(3.0) * t3447 * t17086 + F::new(1.0) * t1161 * (t58023 + t58053 + t58116 + t58129 + t58149 + t58177 + t58200 + t58227) * t1169 + F::cast_from(0.17544670867903938621e1_f64) * t58234 * t1189 + F::new(18.0) * t58237 * t12504 + t57820 + F::cast_from(0.17544670867903938621e1_f64) * t17089 * t3516 + F::cast_from(0.51947577317044391276e2_f64) * t58242 * t3524 + F::cast_from(0.5848223622634646207e0_f64) * t5158 * t12548 + F::cast_from(0.10254018858216406658e4_f64) * t58247 * t12556;
    t58250
}
