//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1122/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1122(t1727: f64, t934: f64, t1045: f64, t14170: f64, t1071: f64, t2630: f64, t3274: f64, t10271: f64, t10282: f64, t10414: f64, t1102: f64, t14115: f64, t14119: f64, t14125: f64, t14127: f64, t14129: f64, t14134: f64, t14137: f64, t14141: f64, t14145: f64, t14150: f64, t14155: f64, t14158: f64, t14161: f64, t14164: f64, t14168: f64, t4587: f64, t4768: f64, t975: f64) -> f64 {
    let t14171 = t1727 * t934;
    let t14172 = t14171 * t1045;
    let t14173 = t14170 * t14172;
    let t14177 = t1727 * t1071 * t2630;
    let t14178 = t3274 * t14177;
    let t14181 = t10271 - 0.14600954814814814815e-3_f64 * t14115 - 0.36958666875e-3_f64 * t1102 * t14119 - 8.0_f64 * t975 * t4768 + t14125 - t14127 + 0.26281718666666666666e-2_f64 * t10414 * t14129 + 0.1478346675e-2_f64 * t1102 * t14134 - 0.8760572888888888889e-3_f64 * t14137 - 0.13140859333333333333e-2_f64 * t1102 * t14141 - 0.65704296666666666667e-3_f64 * t1102 * t14145 - 0.10950716111111111111e-2_f64 * t1102 * t14150 - 0.19711289e-2_f64 * t1102 * t14155 - 0.65704296666666666666e-2_f64 * t1102 * t14158 - 0.13140859333333333333e-2_f64 * t1102 * t14161 - 0.52563437333333333332e-2_f64 * t4587 * t14164 + t14168 + 0.492782225e-3_f64 * t10282 + 0.59133867e-2_f64 * t1102 * t14173 + 0.13140859333333333333e-2_f64 * t1102 * t14178;
    t14181
}
