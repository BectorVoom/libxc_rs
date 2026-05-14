//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 562/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk562<F: Float>(t1089: F, t153: F, t474: F, t1125: F, t678: F, t1298: F, t1386: F, t1394: F, t511: F, t1393: F, t514: F, t185: F, t1301: F, t493: F, t1288: F, t548: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3375 = t153 * t474 * t1089;
    let t3378 = t153 * t1125 * t678;
    let t3380 = t1298 * t1386;
    let t3381 = 16.0 / 15.0 * t3380;
    let t3383 = 4.0 / 5.0 * t511 * t1394;
    let t3384 = t514 * t1393;
    let t3385 = t185 * t3384;
    let t3386 = 8.0 / 15.0 * t3385;
    let t3387 = t514 * t1301;
    let t3388 = t493 * t3387;
    let t3389 = 8.0 / 15.0 * t3388;
    let t3390 = t514 * t1288;
    let t3391 = t548 * t3390;
    (t3375, t3378, t3380, t3381, t3383, t3384, t3385, t3386, t3387, t3388, t3389, t3390, t3391)
}
