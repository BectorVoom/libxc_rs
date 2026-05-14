//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 617/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk617<F: Float>(t1064: F, t391: F, t1070: F, t358: F, t1039: F, t339: F, t344: F, t1037: F, t390: F, t960: F, t40: F, t1191: F, t169: F, t301: F, t678: F, t1: F, t1697: F, t431: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3175 = t1064 * t391;
    let t3176 = 60.0 * t3175;
    let t3177 = t1070 * t358;
    let t3179 = t1064 * t358;
    let t3181 = t339 * t1039;
    let t3183 = t344 * t1039;
    let t3185 = t339 * t1037;
    let t3187 = t344 * t1037;
    let t3191 = t960 * t390;
    let t3192 = t40 * t3191;
    let t3203 = t169 * t1191 * t678 * t301;
    let t3210 = t431 * t1697 * t1;
    (t3175, t3176, t3177, t3179, t3181, t3183, t3185, t3187, t3191, t3192, t3203, t3210)
}
