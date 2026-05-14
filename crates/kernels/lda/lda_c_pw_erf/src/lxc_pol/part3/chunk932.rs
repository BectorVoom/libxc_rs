//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 932/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk932<F: Float>(t3899: F, t5321: F, t571: F, t3663: F, t822: F, t1294: F, t1960: F, t108: F, t2075: F, t267: F, t3979: F, t1278: F, t4488: F, t4495: F, t6710: F, t1390: F, t1458: F) -> (F, F, F, F, F, F) {
    let t12307 = t571 * t3899 * t5321;
    let t12308 = 16.0 / 15.0 * t12307;
    let t12309 = t822 * t3663;
    let t12310 = 4.0 / 45.0 * t12309;
    let t12311 = t1960 * t1294;
    let t12312 = 8.0 / 15.0 * t12311;
    let t12314 = t2075 * t108 * t267;
    let t12316 = 16.0 / 15.0 * t12314 * t3979;
    let t12320 = 8.0 / 15.0 * t4488 * t6710 * t4495 * t1278;
    let t12321 = t1458 * t1390;
    (t12308, t12310, t12312, t12316, t12320, t12321)
}
