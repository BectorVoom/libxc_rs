//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 833/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk833<F: Float>(t3576: F, t565: F, t1288: F, t1518: F, t548: F, t2070: F, t594: F, t211: F, t3663: F, t1279: F, t185: F, t1298: F, t3550: F, t1301: F, t493: F, t543: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9928 = t565 * t3576;
    let t9931 = t548 * t1518 * t1288;
    let t9933 = t2070 * t594;
    let t9934 = t211 * t9933;
    let t9936 = t565 * t3663;
    let t9939 = t185 * t1518 * t1279;
    let t9941 = t1298 * t3550;
    let t9944 = t493 * t1518 * t1301;
    let t9946 = t2070 * t543;
    (t9928, t9931, t9933, t9934, t9936, t9939, t9941, t9944, t9946)
}
