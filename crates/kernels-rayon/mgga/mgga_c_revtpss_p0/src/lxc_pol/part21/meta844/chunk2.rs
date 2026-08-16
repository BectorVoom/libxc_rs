//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3159/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3159(t1179: f64, t16831: f64, t1744: f64, t3477: f64, t3520: f64, t5155: f64, t12552: f64, t1749: f64, t1161: f64, t1169: f64, t1189: f64, t12418: f64, t12473: f64, t12504: f64, t12548: f64, t12556: f64, t17086: f64, t17089: f64, t1745: f64, t3447: f64, t3516: f64, t3524: f64, t45181: f64, t5143: f64, t5158: f64, t57808: f64, t57814: f64, t57816: f64, t57820: f64, t58005: f64, t58023: f64, t58053: f64, t58116: f64, t58129: f64, t58149: f64, t58177: f64, t58200: f64, t58227: f64) -> f64 {
    let t58234 = t16831 * t1179;
    let t58237 = t3477 * t1744;
    let t58242 = t5155 * t3520;
    let t58247 = t1749 * t12552;
    let t58250 = t57808 + t57814 - t57816 + 0.2069040516770936012e4_f64 * t58005 * t12473 + 1.0_f64 * t45181 * t1745 + 3.0_f64 * t12418 * t5143 + 3.0_f64 * t3447 * t17086 + 1.0_f64 * t1161 * (t58023 + t58053 + t58116 + t58129 + t58149 + t58177 + t58200 + t58227) * t1169 + 0.17544670867903938621e1_f64 * t58234 * t1189 + 18.0_f64 * t58237 * t12504 + t57820 + 0.17544670867903938621e1_f64 * t17089 * t3516 + 0.51947577317044391276e2_f64 * t58242 * t3524 + 0.5848223622634646207e0_f64 * t5158 * t12548 + 0.10254018858216406658e4_f64 * t58247 * t12556;
    t58250
}
