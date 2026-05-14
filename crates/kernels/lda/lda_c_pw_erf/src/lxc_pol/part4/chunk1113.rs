//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1113/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1113<F: Float>(t571: F, t6271: F, t9678: F, t1333: F, t2466: F, t3867: F, t951: F, t1318: F, t3854: F, t6276: F, t2478: F, t1319: F, t3802: F, t519: F, t6281: F, t1315: F, t6198: F) -> (F, F, F, F, F, F) {
    let t16245 = t571 * t9678 * t6271;
    let t16246 = 32.0 / 135.0 * t16245;
    let t16251 = 16.0 / 45.0 * t571 * t3867 * t2466 * t1333 * t951;
    let t16253 = t1318 * t3854 * t6276;
    let t16254 = 32.0 / 135.0 * t16253;
    let t16255 = t2478 * t1333;
    let t16259 = 16.0 / 45.0 * t1318 * t1319 * t16255 * t951;
    let t16261 = t519 * t3802 * t6281;
    let t16262 = 16.0 / 135.0 * t16261;
    let t16264 = 8.0 / 45.0 * t6198 * t1315;
    (t16246, t16251, t16254, t16259, t16262, t16264)
}
