//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 148/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk148<F: Float>(t110: F, t56: F, t69: F, t352: F, t355: F, t362: F) -> (F, F) {
    let t381 = F::cast_from(0.28737583333333333_f64) * t69 * t110 * t56;
    let t384 = -t352 - t355 - t381 - F::cast_from(1.724255_f64) * t69 * t362;
    (t381, t384)
}
