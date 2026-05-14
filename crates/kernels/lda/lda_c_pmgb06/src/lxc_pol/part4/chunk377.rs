//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 377/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk377<F: Float>(t1409: F, t83: F, t188: F, t12: F, t158: F) -> (F, F, F, F) {
    let t1410 = t83 * t1409;
    let t1412 = 4.0 / 3.0 * t1410 * t188;
    let t1413 = t158 * t12;
    let t1414 = 1.0 / t1413;
    (t1410, t1412, t1413, t1414)
}
