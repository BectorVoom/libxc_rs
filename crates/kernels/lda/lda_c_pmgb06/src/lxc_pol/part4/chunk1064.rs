//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1064/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1064<F: Float>(t15893: F, t5211: F, t6372: F, t2497: F, t3226: F, t2501: F, t3220: F, t1972: F, t5494: F, t13933: F, t439: F, t5272: F, t493: F, t5276: F, t5486: F, t12617: F, t5281: F) -> (F, F, F, F, F, F, F, F) {
    let t15894 = 8.0 / 135.0 * t15893;
    let t15895 = t5211 * t6372;
    let t15896 = 4.0 / 27.0 * t15895;
    let t15897 = t3226 * t2497;
    let t15898 = 8.0 / 135.0 * t15897;
    let t15899 = t3220 * t2501;
    let t15900 = 8.0 / 135.0 * t15899;
    let t15902 = 4.0 / 45.0 * t1972 * t5494;
    let t15905 = 2.0 / 27.0 * t439 * t13933 * t5272;
    let t15908 = 2.0 / 45.0 * t493 * t5486 * t5276;
    let t15911 = 2.0 / 27.0 * t493 * t12617 * t5281;
    (t15894, t15896, t15898, t15900, t15902, t15905, t15908, t15911)
}
