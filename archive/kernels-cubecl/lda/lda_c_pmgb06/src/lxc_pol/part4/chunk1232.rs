//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1232/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1232<F: Float>(t1069: F, t1438: F, t2604: F, t439: F, t9084: F, t1074: F, t2864: F, t6522: F, t432: F, t6836: F, t132: F, t435: F, t6674: F) -> (F, F, F, F) {
    let t16228 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t439 * t9084 * t2604 * t1438 * t1069;
    let t16237 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t439 * t2864 * t6522 * t1074;
    let t16238 = t432 * t6836;
    let t16239 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t16238;
    let t16241 = t132 * t435 * t6674;
    (t16228, t16237, t16239, t16241)
}
