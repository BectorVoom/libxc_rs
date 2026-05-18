//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1405/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1405<F: Float>(t16402: F, t16404: F, t16406: F, t16408: F, t16411: F, t16412: F, t16413: F, t16414: F, t16415: F, t16416: F, t16419: F, t16421: F, t16424: F, t16425: F, t16427: F) -> F {
    let t18234 = -t16402 - t16404 - t16406 - t16408 - t16411 - t16412 + t16413 - t16414 - t16415 - t16416 + t16419 + t16421 - t16424 - t16425 + t16427;
    t18234
}
