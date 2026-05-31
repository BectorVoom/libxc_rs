//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 159/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk159<F: Float>(t107: F, t402: F, t405: F, t410: F, t81: F, t93: F) -> F {
    let t413 = t93 * t402 / F::cast_from(12.0_f64) - F::cast_from(0.013655_f64) * t405 + F::cast_from(0.0030486129349252553_f64) * t81 - F::cast_from(0.00046475_f64) * t107 * t410;
    t413
}
