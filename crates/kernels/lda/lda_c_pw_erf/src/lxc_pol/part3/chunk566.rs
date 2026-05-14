//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 566/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk566<F: Float>(t1322: F, t3416: F, t1287: F, t558: F, t352: F, t1319: F, t1318: F, t1320: F, t954: F, t1351: F, t549: F, t951: F, t2017: F, t1529: F, t565: F, t1524: F, t568: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3418 = 16.0 / 15.0 * t3416 * t1322;
    let t3419 = t1287 * t558;
    let t3420 = t3419 * t352;
    let t3421 = t1319 * t3420;
    let t3423 = 8.0 / 15.0 * t1318 * t3421;
    let t3424 = t1320 * t954;
    let t3425 = t1319 * t3424;
    let t3427 = 8.0 / 15.0 * t1318 * t3425;
    let t3429 = t549 * t1351 * t951;
    let t3430 = t2017 * t3429;
    let t3432 = 8.0 / 9.0 * t1318 * t3430;
    let t3433 = t565 * t1529;
    let t3434 = 4.0 / 45.0 * t3433;
    let t3435 = t1524 * t568;
    (t3418, t3420, t3421, t3423, t3424, t3425, t3427, t3429, t3430, t3432, t3433, t3434, t3435)
}
