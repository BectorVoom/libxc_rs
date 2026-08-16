//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 356/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk356<F: Float>(t1289: F, t1291: F, t1296: F, t1297: F, t1309: F, t378: F, t384: F, t74: F, t387: F) -> (F, F) {
    let t1311 = t1289 * t74 - F::cast_from(2.0_f64) * t1291 * t384 + F::cast_from(2.0_f64) * t1296 * t1297 - t378 * t1309;
    let t1312 = t1311 * t387;
    (t1311, t1312)
}
