//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1122/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1122<F: Float>(t1727: F, t934: F, t1045: F, t14170: F, t1071: F, t2630: F, t3274: F, t10271: F, t10282: F, t10414: F, t1102: F, t14115: F, t14119: F, t14125: F, t14127: F, t14129: F, t14134: F, t14137: F, t14141: F, t14145: F, t14150: F, t14155: F, t14158: F, t14161: F, t14164: F, t14168: F, t4587: F, t4768: F, t975: F) -> F {
    let t14171 = t1727 * t934;
    let t14172 = t14171 * t1045;
    let t14173 = t14170 * t14172;
    let t14177 = t1727 * t1071 * t2630;
    let t14178 = t3274 * t14177;
    let t14181 = t10271 - F::cast_from(0.14600954814814814815e-3_f64) * t14115 - F::cast_from(0.36958666875e-3_f64) * t1102 * t14119 - F::cast_from(8.0_f64) * t975 * t4768 + t14125 - t14127 + F::cast_from(0.26281718666666666666e-2_f64) * t10414 * t14129 + F::cast_from(0.1478346675e-2_f64) * t1102 * t14134 - F::cast_from(0.8760572888888888889e-3_f64) * t14137 - F::cast_from(0.13140859333333333333e-2_f64) * t1102 * t14141 - F::cast_from(0.65704296666666666667e-3_f64) * t1102 * t14145 - F::cast_from(0.10950716111111111111e-2_f64) * t1102 * t14150 - F::cast_from(0.19711289e-2_f64) * t1102 * t14155 - F::cast_from(0.65704296666666666666e-2_f64) * t1102 * t14158 - F::cast_from(0.13140859333333333333e-2_f64) * t1102 * t14161 - F::cast_from(0.52563437333333333332e-2_f64) * t4587 * t14164 + t14168 + F::cast_from(0.492782225e-3_f64) * t10282 + F::cast_from(0.59133867e-2_f64) * t1102 * t14173 + F::cast_from(0.13140859333333333333e-2_f64) * t1102 * t14178;
    t14181
}
