//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 868/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk868<F: Float>(t1423: F, t3936: F, t10967: F, t168: F, t270: F, t2782: F, t671: F, t153: F, t3196: F, t474: F, t2869: F, t678: F, t1210: F, t1534: F, t4107: F, t632: F) -> (F, F, F, F, F, F, F) {
    let t11168 = t1423 * t3936;
    let t11196 = 0.9079060239445599 * t168 * t10967 * t270;
    let t11198 = t168 * t2782 * t671;
    let t11201 = t153 * t474 * t3196;
    let t11204 = t153 * t2869 * t678;
    let t11211 = t168 * t1210 * t1534;
    let t11215 = t4107 * t632;
    (t11168, t11196, t11198, t11201, t11204, t11211, t11215)
}
