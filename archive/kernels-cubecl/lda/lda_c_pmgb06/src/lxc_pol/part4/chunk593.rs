//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 593/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk593<F: Float>(t1601: F, t2599: F, t166: F, t161: F, t822: F) -> (F, F, F, F) {
    let t2600 = t1601 * t2599;
    let t2601 = t166 * t2600;
    let t2603 = t161 * t2601 / F::cast_from(15.0_f64);
    let t2604 = t822 * t822;
    (t2600, t2601, t2603, t2604)
}
