//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1007/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1007<F: Float>(t13480: F, t4565: F, t13475: F, t4579: F, t13511: F, t3255: F, t4597: F, t1035: F, t3293: F, t1727: F, t934: F, t1045: F, t1071: F, t2630: F, t3274: F, t10271: F, t10282: F, t10414: F, t1102: F, t14115: F, t14119: F, t14125: F, t14127: F, t14129: F, t14134: F, t14137: F, t14141: F, t14145: F, t14150: F, t14155: F, t4587: F, t4768: F, t975: F) -> (F,) {
    let t14158 = t4565 * t13480;
    let t14161 = t4579 * t13475;
    let t14164 = t4579 * t13511;
    let t14168 = 0.13140859333333333333e-2 * t3255 * t4597;
    let t14170 = t3293 * t1035;
    let t14171 = t1727 * t934;
    let t14172 = t14171 * t1045;
    let t14173 = t14170 * t14172;
    let t14177 = t1727 * t1071 * t2630;
    let t14178 = t3274 * t14177;
    let t14181 = t10271 - 0.14600954814814814815e-3 * t14115 - 0.36958666875e-3 * t1102 * t14119 - 8.0 * t975 * t4768 + t14125 - t14127 + 0.26281718666666666666e-2 * t10414 * t14129 + 0.1478346675e-2 * t1102 * t14134 - 0.8760572888888888889e-3 * t14137 - 0.13140859333333333333e-2 * t1102 * t14141 - 0.65704296666666666667e-3 * t1102 * t14145 - 0.10950716111111111111e-2 * t1102 * t14150 - 0.19711289e-2 * t1102 * t14155 - 0.65704296666666666666e-2 * t1102 * t14158 - 0.13140859333333333333e-2 * t1102 * t14161 - 0.52563437333333333332e-2 * t4587 * t14164 + t14168 + 0.492782225e-3 * t10282 + 0.59133867e-2 * t1102 * t14173 + 0.13140859333333333333e-2 * t1102 * t14178;
    (t14181,)
}
