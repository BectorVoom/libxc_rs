//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1025/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1025<F: Float>(t5168: F, t5248: F, t2010: F, t4668: F, t5225: F, t132: F, t435: F, t5119: F, t3031: F, t813: F, t137: F, t3033: F) -> (F, F, F, F) {
    let t12186 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t5168 * t5248;
    let t12189 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t2010 * t5225 * t4668;
    let t12191 = t132 * t435 * t5119;
    let t12192 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t12191;
    let t12193 = t813 * t3031;
    let t12197 = t132 * t137 * t12193 * t3033 / F::cast_from(5.0_f64);
    (t12186, t12189, t12192, t12197)
}
