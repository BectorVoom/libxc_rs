//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 978/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk978<F: Float>(t2477: F, t5187: F, t2002: F, t6300: F, t6275: F, t6303: F, t20353: F, t20355: F, t20359: F, t20361: F, t20364: F, t20367: F, t20369: F, t20372: F, t16749: F, t1995: F, t6134: F) -> (F, F, F, F, F, F) {
    let t20374 = 2.0 / 15.0 * t5187 * t2477;
    let t20376 = 2.0 / 15.0 * t2002 * t6300;
    let t20378 = 4.0 / 15.0 * t6275 * t6303;
    let t20379 = t20353 - t20355 + t20359 + t20361 + t20364 + t20367 + t20369 + t20372 + t20374 + t20376 + t20378;
    let t20380 = 4.0 / 135.0 * t16749;
    let t20382 = t6134 * t1995 / 5.0;
    (t20374, t20376, t20378, t20379, t20380, t20382)
}
