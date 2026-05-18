//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1235/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1235<F: Float>(t12403: F, t21468: F, t4488: F, t12113: F, t21410: F, t3974: F, t4475: F, t6408: F, t6413: F, t6748: F, t6379: F, t6752: F) -> (F, F, F, F, F) {
    let t22222 = F::new(16.0) / F::new(15.0) * t4488 * t12403 * t21468;
    let t22225 = F::new(8.0) / F::new(5.0) * t4488 * t12113 * t21410;
    let t22228 = F::new(16.0) / F::new(15.0) * t3974 * t4475 * t6408;
    let t22231 = F::new(16.0) / F::new(5.0) * t3974 * t6748 * t6413;
    let t22234 = F::new(16.0) / F::new(3.0) * t3974 * t6752 * t6379;
    (t22222, t22225, t22228, t22231, t22234)
}
