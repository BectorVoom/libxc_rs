//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 661/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk661<F: Float>(t2429: F, t494: F, t1991: F, t1325: F, t542: F, t3402: F, t519: F, t2325: F, t3476: F, t348: F) -> (F, F, F, F, F, F, F, F) {
    let t6343 = t2429 * t494;
    let t6344 = t1991 * t6343;
    let t6346 = 8.0 / 27.0 * t1325 * t6344;
    let t6347 = t2429 * t542;
    let t6348 = t3402 * t6347;
    let t6350 = 4.0 / 27.0 * t519 * t6348;
    let t6351 = t3476 * t2325;
    let t6352 = t6351 * t348;
    (t6343, t6344, t6346, t6347, t6348, t6350, t6351, t6352)
}
