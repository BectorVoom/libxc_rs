//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 596/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk596<F: Float>(t3239: F, t493: F, t2938: F, t498: F, t496: F, t1414: F, t164: F) -> (F, F, F, F, F) {
    let t3241 = t493 * t3239 / F::new(9.0);
    let t3242 = t498 * t2938;
    let t3243 = t496 * t3242;
    let t3245 = t493 * t3243 / F::new(45.0);
    let t3247 = F::new(1.0) / t164 / t1414;
    (t3241, t3242, t3243, t3245, t3247)
}
