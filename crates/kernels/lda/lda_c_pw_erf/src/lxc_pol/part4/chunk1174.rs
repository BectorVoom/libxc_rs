//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1174/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1174<F: Float>(t17200: F, t17215: F, t17236: F, t17263: F, t17297: F, t17312: F, t17326: F, t17348: F, t185: F, t186: F, t530: F, t1498: F, t2528: F, t2072: F, t5211: F, t6851: F) -> (F, F, F, F) {
    let t17355 = 2.0 / 15.0 * t185 * t186 * t530 * (t17200 + t17215 + t17236 + t17263 + t17297 + t17312 + t17326 + t17348);
    let t17357 = 2.0 / 15.0 * t1498 * t2528;
    let t17359 = 16.0 / 15.0 * t5211 * t2072;
    let t17361 = 16.0 / 15.0 * t6851 * t2072;
    (t17355, t17357, t17359, t17361)
}
