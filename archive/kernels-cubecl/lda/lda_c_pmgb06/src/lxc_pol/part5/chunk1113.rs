//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1113/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1113<F: Float>(t6275: F, t6303: F, t20353: F, t20355: F, t20359: F, t20361: F, t20364: F, t20367: F, t20369: F, t20372: F, t20374: F, t20376: F) -> (F, F) {
    let t20378 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t6275 * t6303;
    let t20379 = t20353 - t20355 + t20359 + t20361 + t20364 + t20367 + t20369 + t20372 + t20374 + t20376 + t20378;
    (t20378, t20379)
}
