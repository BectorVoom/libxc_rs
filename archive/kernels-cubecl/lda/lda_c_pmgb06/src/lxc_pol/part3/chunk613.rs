//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 613/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk613<F: Float>(t161: F, t3453: F, t516: F) -> (F, F, F, F) {
    let t3454 = t161 * t3453;
    let t3455 = t3454 / F::cast_from(15.0_f64);
    let t3456 = t516 * t516;
    let t3457 = F::cast_from(1.0_f64) / t3456;
    (t3454, t3455, t3456, t3457)
}
