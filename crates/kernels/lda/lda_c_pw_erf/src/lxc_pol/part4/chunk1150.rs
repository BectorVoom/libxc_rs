//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1150/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1150<F: Float>(t519: F, t6347: F, t9723: F, t1446: F, t6348: F, t1278: F, t2429: F, t3402: F, t6353: F, t15802: F, t1991: F, t1318: F, t3854: F, t6255: F, t3802: F, t6326: F) -> (F, F, F, F, F, F, F) {
    let t16935 = t519 * t9723 * t6347;
    let t16936 = 16.0 / 81.0 * t16935;
    let t16938 = 8.0 / 27.0 * t1446 * t6348;
    let t16942 = 4.0 / 27.0 * t519 * t3402 * t2429 * t1278;
    let t16944 = 16.0 / 9.0 * t1446 * t6353;
    let t16947 = 8.0 / 9.0 * t519 * t1991 * t15802;
    let t16949 = t1318 * t3854 * t6255;
    let t16950 = 32.0 / 135.0 * t16949;
    let t16952 = t519 * t3802 * t6326;
    (t16936, t16938, t16942, t16944, t16947, t16950, t16952)
}
