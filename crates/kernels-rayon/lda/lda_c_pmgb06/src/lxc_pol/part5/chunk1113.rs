//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1113/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1113(t6275: f64, t6303: f64, t20353: f64, t20355: f64, t20359: f64, t20361: f64, t20364: f64, t20367: f64, t20369: f64, t20372: f64, t20374: f64, t20376: f64) -> (f64, f64) {
    let t20378 = 4.0_f64 / 15.0_f64 * t6275 * t6303;
    let t20379 = t20353 - t20355 + t20359 + t20361 + t20364 + t20367 + t20369 + t20372 + t20374 + t20376 + t20378;
    (t20378, t20379)
}
