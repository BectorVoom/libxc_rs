//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1030/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1030<F: Float>(t12239: F, t3043: F, t831: F, t3461: F, t3450: F, t132: F, t435: F, t4965: F, t432: F, t5120: F, t1592: F, t1872: F) -> (F, F, F, F, F, F, F) {
    let t12240 = t12239 / F::cast_from(45.0_f64);
    let t12241 = t831 * t3043;
    let t12242 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t12241;
    let t12244 = t831 * t3461 / F::cast_from(5.0_f64);
    let t12245 = t831 * t3450;
    let t12246 = t12245 / F::cast_from(45.0_f64);
    let t12248 = t132 * t435 * t4965;
    let t12249 = t12248 / F::cast_from(15.0_f64);
    let t12251 = t432 * t5120 / F::cast_from(5.0_f64);
    let t12252 = t1872 * t1592;
    (t12240, t12242, t12244, t12246, t12249, t12251, t12252)
}
