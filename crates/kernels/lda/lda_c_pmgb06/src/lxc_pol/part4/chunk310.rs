//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 310/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk310<F: Float>(t1004: F, t1009: F, t1012: F, t1017: F, t1021: F, t1028: F, t1038: F, t1046: F, t1050: F, t1054: F, t1055: F, t1058: F, t1061: F, t1062: F, t138: F, t269: F, t282: F, t30: F, t410: F, t661: F, t668: F, t676: F, t683: F, t986: F, t992: F, t994: F) -> F {
    let t1065 = -F::cast_from(0.0007098352262222222_f64) * t30 * t410 * t269 - F::cast_from(0.03424666666666667_f64) * t138 * t986 * t668 - F::new(2.0) * t992 * t994 + F::new(1.0) * t661 * t1004 + F::cast_from(32.16395899738507_f64) * t1009 * t1012 + t1017 + t1021 + t1028 - t1038 - t1046 - F::cast_from(0.00024415263074675396_f64) * t30 * t410 * t282 - F::cast_from(0.01084358130030174_f64) * t138 * t1050 * t683 - F::cast_from(1.1696447245269292_f64) * t1054 * t1055 + F::cast_from(0.5848223622634646_f64) * t676 * t1058 + F::cast_from(17.315859105681465_f64) * t1061 * t1062;
    t1065
}
