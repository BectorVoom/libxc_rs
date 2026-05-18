//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 921/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk921<F: Float>(t1284: F, t3660: F, t3576: F, t565: F, t1288: F, t1518: F, t548: F, t2070: F, t594: F, t211: F, t3663: F, t1279: F, t185: F) -> (F, F, F, F, F, F, F) {
    let t9925 = t1284 * t3660;
    let t9928 = t565 * t3576;
    let t9931 = t548 * t1518 * t1288;
    let t9933 = t2070 * t594;
    let t9934 = t211 * t9933;
    let t9936 = t565 * t3663;
    let t9939 = t185 * t1518 * t1279;
    (t9925, t9928, t9931, t9933, t9934, t9936, t9939)
}
