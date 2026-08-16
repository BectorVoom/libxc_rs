//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 310/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk310(t1004: f64, t1009: f64, t1012: f64, t1017: f64, t1021: f64, t1028: f64, t1038: f64, t1046: f64, t1050: f64, t1054: f64, t1055: f64, t1058: f64, t1061: f64, t1062: f64, t138: f64, t269: f64, t282: f64, t30: f64, t410: f64, t661: f64, t668: f64, t676: f64, t683: f64, t986: f64, t992: f64, t994: f64) -> f64 {
    let t1065 = -0.0007098352262222222_f64 * t30 * t410 * t269 - 0.03424666666666667_f64 * t138 * t986 * t668 - 2.0_f64 * t992 * t994 + 1.0_f64 * t661 * t1004 + 32.16395899738507_f64 * t1009 * t1012 + t1017 + t1021 + t1028 - t1038 - t1046 - 0.00024415263074675396_f64 * t30 * t410 * t282 - 0.01084358130030174_f64 * t138 * t1050 * t683 - 1.1696447245269292_f64 * t1054 * t1055 + 0.5848223622634646_f64 * t676 * t1058 + 17.315859105681465_f64 * t1061 * t1062;
    t1065
}
