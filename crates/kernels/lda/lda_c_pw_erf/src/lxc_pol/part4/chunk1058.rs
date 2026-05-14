//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1058/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1058<F: Float>(t8189: F, t11327: F, t19: F, t729: F, t7307: F, t734: F, t11330: F, t8193: F, t8195: F, t11335: F, t11337: F, t11339: F, t1746: F, t7314: F, t8199: F, t8204: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t15408 = 1.169644679491041 * t8189;
    let t15410 = 96.0 * t11327;
    let t15413 = t7307 * t729 * t19 * t734;
    let t15415 = 0.0003662311007350632 * t11330;
    let t15416 = 192.0 * t8193;
    let t15417 = 48.0 * t8195;
    let t15418 = 7.017868076946245 * t11335;
    let t15419 = 64.0 * t11337;
    let t15420 = 48.0 * t11339;
    let t15421 = t7314 * t1746;
    let t15423 = 96.0 * t8199;
    let t15424 = 160.0 * t8204;
    (t15408, t15410, t15413, t15415, t15416, t15417, t15418, t15419, t15420, t15421, t15423, t15424)
}
