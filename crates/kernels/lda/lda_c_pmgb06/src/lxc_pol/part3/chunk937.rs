//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 937/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk937<F: Float>(t117: F, t123: F, t191: F, t4001: F, t1100: F, t290: F, t395: F, t3974: F, t199: F, t4297: F, t1126: F, t247: F) -> (F, F, F, F, F) {
    let t10886 = F::cast_from(0.4097848972398244_f64) * t123 * t4001 * t191 * t117;
    let t10895 = F::cast_from(6.399008129061525_f64) * t1100 * t290;
    let t10896 = t395 * t3974;
    let t10902 = F::cast_from(2.4210827305188265_f64) * t123 * t4297 * t199;
    let t10903 = t247 * t1126;
    (t10886, t10895, t10896, t10902, t10903)
}
