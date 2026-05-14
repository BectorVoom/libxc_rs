//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 758/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk758<F: Float>(t549: F, t833: F, t593: F, t5269: F, t1318: F, t2005: F, t945: F, t1326: F, t1325: F, t1319: F, t4684: F, t571: F, t2034: F, t3854: F, t1403: F, t816: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5270 = t833 * t549;
    let t5271 = t5270 * t593;
    let t5272 = t5269 * t5271;
    let t5274 = 16.0 / 15.0 * t1318 * t5272;
    let t5275 = t2005 * t945;
    let t5276 = t1326 * t5275;
    let t5278 = 8.0 / 45.0 * t1325 * t5276;
    let t5279 = t1319 * t4684;
    let t5281 = 8.0 / 15.0 * t571 * t5279;
    let t5282 = t3854 * t2034;
    let t5284 = 32.0 / 135.0 * t1318 * t5282;
    let t5285 = t816 * t1403;
    (t5270, t5271, t5272, t5274, t5275, t5276, t5278, t5279, t5281, t5282, t5284, t5285)
}
