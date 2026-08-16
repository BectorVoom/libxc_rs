//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 868/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk868<F: Float>(t493: F, t6120: F, t2582: F, t441: F, t445: F, t439: F, t224: F, t2591: F) -> (F, F, F, F, F) {
    let t6122 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t493 * t6120;
    let t6123 = t441 * t2582;
    let t6124 = t6123 * t445;
    let t6126 = t439 * t6124 / F::cast_from(45.0_f64);
    let t6127 = t2591 * t224;
    (t6122, t6123, t6124, t6126, t6127)
}
