//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 727/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk727<F: Float>(t3231: F, t3233: F, t3272: F, t3274: F, t1441: F, t2002: F, t3177: F, t806: F, t1420: F, t2007: F, t1980: F, t431: F) -> (F, F, F, F, F, F, F, F) {
    let t5158 = 2.0 / 135.0 * t3231;
    let t5159 = 2.0 / 81.0 * t3233;
    let t5160 = 2.0 / 135.0 * t3272;
    let t5161 = 2.0 / 81.0 * t3274;
    let t5163 = t2002 * t1441 / 27.0;
    let t5165 = t3177 * t806 / 45.0;
    let t5167 = 2.0 / 45.0 * t1420 * t2007;
    let t5168 = t431 * t1980;
    (t5158, t5159, t5160, t5161, t5163, t5165, t5167, t5168)
}
