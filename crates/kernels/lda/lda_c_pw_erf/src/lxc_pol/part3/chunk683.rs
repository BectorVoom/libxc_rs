//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 683/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk683<F: Float>(t203: F, t4701: F, t184: F, t221: F, t4039: F, t493: F, t3881: F, t3885: F, t3901: F, t462: F, t2849: F, t2852: F) -> (F, F, F, F, F, F, F, F) {
    let t4702 = t203 * t4701;
    let t4703 = t4702 * t184;
    let t4705 = 2.0 / 15.0 * t4703 * t221;
    let t4707 = 4.0 / 15.0 * t493 * t4039;
    let t4708 = 8.0 / 135.0 * t3881;
    let t4709 = 8.0 / 81.0 * t3885;
    let t4710 = 16.0 / 45.0 * t3901;
    let t4711 = 4.0 * t462;
    let t4712 = 12.0 * t2849;
    let t4713 = -t4711 - t4712 + t2852;
    (t4702, t4703, t4705, t4707, t4708, t4709, t4710, t4713)
}
