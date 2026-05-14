//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 842/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk842<F: Float>(t2429: F, t494: F, t1991: F, t1325: F, t542: F, t3402: F, t519: F, t2325: F, t3476: F, t348: F, t2419: F, t593: F, t1308: F, t571: F, t1333: F, t2337: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6343 = t2429 * t494;
    let t6344 = t1991 * t6343;
    let t6346 = 8.0 / 27.0 * t1325 * t6344;
    let t6347 = t2429 * t542;
    let t6348 = t3402 * t6347;
    let t6350 = 4.0 / 27.0 * t519 * t6348;
    let t6351 = t3476 * t2325;
    let t6352 = t6351 * t348;
    let t6353 = t1991 * t6352;
    let t6355 = 8.0 / 9.0 * t519 * t6353;
    let t6356 = t2419 * t593;
    let t6357 = t1308 * t6356;
    let t6359 = 4.0 / 45.0 * t571 * t6357;
    let t6360 = t1333 * t2337;
    (t6343, t6344, t6346, t6347, t6348, t6350, t6351, t6352, t6353, t6355, t6356, t6357, t6359, t6360)
}
