//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1059/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1059<F: Float>(t2002: F, t5273: F, t1444: F, t6770: F, t10293: F, t493: F, t6769: F, t1586: F, t2541: F, t2991: F, t5499: F, t6536: F, t12912: F, t835: F, t15801: F, t15802: F, t15803: F, t15804: F, t15805: F, t15808: F, t15810: F, t15815: F, t15817: F) -> (F, F, F, F, F, F, F) {
    let t15819 = 2.0 / 27.0 * t2002 * t5273;
    let t15821 = 2.0 / 27.0 * t1444 * t6770;
    let t15824 = 2.0 / 27.0 * t493 * t10293 * t6769;
    let t15828 = t493 * t2991 * t2541 * t1586 / 27.0;
    let t15829 = t5499 * t6536;
    let t15830 = 4.0 / 27.0 * t15829;
    let t15831 = t12912 * t835;
    let t15832 = 8.0 / 135.0 * t15831;
    let t15833 = t15801 + t15802 - t15803 - t15804 + t15805 - t15808 + t15810 - t15815 - t15817 - t15819 - t15821 - t15824 - t15828 + t15830 + t15832;
    (t15819, t15821, t15824, t15828, t15830, t15832, t15833)
}
