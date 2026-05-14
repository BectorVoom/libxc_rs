//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 993/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk993<F: Float>(t2954: F, t3402: F, t3476: F, t519: F, t806: F, t1472: F, t5279: F, t2967: F, t4665: F, t2017: F, t571: F, t4675: F, t951: F, t4758: F, t3416: F, t5282: F) -> (F, F, F, F, F, F, F) {
    let t13287 = 8.0 / 9.0 * t519 * t3402 * t806 * t3476 * t2954;
    let t13289 = 8.0 / 5.0 * t1472 * t5279;
    let t13290 = t4665 * t2967;
    let t13293 = 16.0 / 3.0 * t571 * t2017 * t13290;
    let t13294 = t4675 * t951;
    let t13297 = 16.0 / 5.0 * t571 * t4758 * t13294;
    let t13298 = t3416 * t5282;
    (t13287, t13289, t13290, t13293, t13294, t13297, t13298)
}
