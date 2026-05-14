//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1098/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1098<F: Float>(t12621: F, t12623: F, t12625: F, t12631: F, t13100: F, t493: F, t834: F, t2462: F, t3198: F, t16396: F, t16398: F, t16400: F, t16402: F, t16404: F, t16406: F, t16408: F, t16411: F, t16412: F) -> (F, F, F, F, F, F, F) {
    let t16413 = 8.0 / 405.0 * t12621;
    let t16414 = 8.0 / 135.0 * t12623;
    let t16415 = 8.0 / 135.0 * t12625;
    let t16416 = 8.0 / 135.0 * t12631;
    let t16419 = 2.0 / 45.0 * t493 * t13100 * t834;
    let t16421 = 2.0 / 45.0 * t3198 * t2462;
    let t16422 = -t16396 - t16398 - t16400 - t16402 - t16404 - t16406 - t16408 - t16411 - t16412 + t16413 - t16414 - t16415 - t16416 + t16419 + t16421;
    (t16413, t16414, t16415, t16416, t16419, t16421, t16422)
}
