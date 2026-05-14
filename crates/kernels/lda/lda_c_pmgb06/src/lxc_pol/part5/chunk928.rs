//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 928/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk928<F: Float>(t19506: F, t11884: F, t9350: F, t9352: F, t11882: F, t19485: F, t19488: F, t19493: F, t19497: F, t19498: F, t19499: F, t19504: F, t1444: F, t7663: F, t441: F, t7501: F) -> (F, F, F, F, F, F, F) {
    let t19507 = 2.0 / 15.0 * t19506;
    let t19508 = 4.0 / 135.0 * t11884;
    let t19509 = 4.0 / 405.0 * t9350;
    let t19510 = 4.0 / 405.0 * t9352;
    let t19511 = -t19485 - t19488 + t19493 - t19497 + t19498 + t19499 + t19504 + t11882 - t19507 + t19508 + t19509 + t19510;
    let t19514 = t1444 * t7663 / 15.0;
    let t19515 = t441 * t7501;
    (t19507, t19508, t19509, t19510, t19511, t19514, t19515)
}
