//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1071/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1071<F: Float>(t8278: F, t8286: F, t8291: F, t11369: F, t20033: F, t85: F, t15450: F, t402: F, t7376: F, t75: F, t8303: F, t15453: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20057 = F::cast_from(0.032530742648344574_f64) * t8278;
    let t20058 = F::cast_from(0.016265371324172287_f64) * t8286;
    let t20059 = F::cast_from(0.4815944609513912_f64) * t8291;
    let t20060 = F::cast_from(3076.1691063023386_f64) * t11369;
    let t20062 = F::cast_from(0.019751789702565206_f64) * t20033 * t85;
    let t20063 = F::new(3.0) * t15450;
    let t20066 = t7376 * t75 * t402;
    let t20067 = F::cast_from(0.5848223397455204_f64) * t20066;
    let t20068 = F::cast_from(103.89453539625518_f64) * t8303;
    let t20069 = F::new(24.0) * t15453;
    (t20057, t20058, t20059, t20060, t20062, t20063, t20067, t20068, t20069)
}
