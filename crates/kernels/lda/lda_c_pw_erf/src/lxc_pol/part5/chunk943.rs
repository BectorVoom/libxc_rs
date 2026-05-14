//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 943/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk943<F: Float>(t8278: F, t8286: F, t8291: F, t11369: F, t20033: F, t85: F, t15450: F, t402: F, t7376: F, t75: F, t8303: F, t15453: F, t15455: F, t11372: F, t11374: F, t11376: F, t8285: F, t8290: F, t8296: F, t8300: F, t8301: F, t8356: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t20057 = 0.032530742648344574 * t8278;
    let t20058 = 0.016265371324172287 * t8286;
    let t20059 = 0.4815944609513912 * t8291;
    let t20060 = 3076.1691063023386 * t11369;
    let t20062 = 0.019751789702565206 * t20033 * t85;
    let t20063 = 3.0 * t15450;
    let t20066 = t7376 * t75 * t402;
    let t20067 = 0.5848223397455204 * t20066;
    let t20068 = 103.89453539625518 * t8303;
    let t20069 = 24.0 * t15453;
    let t20070 = 60.0 * t15455;
    let t20071 = -t20057 + t8285 + t20058 + t8290 + t20059 - t8296 - t20060 - t11372 - t11374 + t20062 - t8300 + t20063 + t11376 - 1.825614615114074 * t8301 - t20067 + t20068 - t8356 - t20069 + t20070;
    (t20057, t20058, t20059, t20060, t20062, t20063, t20067, t20068, t20069, t20070, t20071)
}
