//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 306/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk306<F: Float>(t1025: F, t1043: F, t1041: F, t109: F, t675: F, t273: F, t978: F, t682: F, t964: F, t957: F, t963: F, t967: F, t1004: F, t1009: F, t1012: F, t1017: F, t1021: F, t1028: F, t1038: F, t138: F, t269: F, t282: F, t30: F, t410: F, t661: F, t668: F, t676: F, t683: F, t986: F, t992: F, t994: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1044 = t1025 * t1043;
    let t1046 = 16.081979498692537 * t1041 * t1044;
    let t1050 = t109 * t675;
    let t1054 = t273 * t978;
    let t1055 = t964 * t682;
    let t1058 = t957 * t682;
    let t1061 = t273 * t963;
    let t1062 = t964 * t967;
    let t1065 = -0.0007098352262222222 * t30 * t410 * t269 - 0.03424666666666667 * t138 * t986 * t668 - 2.0 * t992 * t994 + 1.0 * t661 * t1004 + 32.16395899738507 * t1009 * t1012 + t1017 + t1021 + t1028 - t1038 - t1046 - 0.00024415263074675396 * t30 * t410 * t282 - 0.01084358130030174 * t138 * t1050 * t683 - 1.1696447245269292 * t1054 * t1055 + 0.5848223622634646 * t676 * t1058 + 17.315859105681465 * t1061 * t1062;
    (t1044, t1046, t1050, t1054, t1055, t1058, t1061, t1062, t1065)
}
