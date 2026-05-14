//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1028/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1028<F: Float>(t405: F, t7853: F, t103: F, t13399: F, t13407: F, t13565: F, t14106: F, t14110: F, t14152: F, t14162: F, t14170: F, t14181: F, t14183: F, t1576: F, t17177: F, t17185: F, t17215: F, t17217: F, t17222: F, t17224: F, t19314: F, t19349: F, t19354: F, t19381: F, t2060: F, t3358: F, t525: F, t9552: F) -> (F,) {
    let t21216 = t405 * t7853;
    let t21218 = 0.21595 * t17177 - 0.02666666666666667 * t17185 + 0.08 * t17215 + 0.10666666666666667 * t17217 - 0.02666666666666667 * t17222 + 0.0044444444444444444 * t17224 + 0.09597777777777777 * t13399 + 0.11197407407407407 * t13407 - 0.12 * t13565 * t14106 * t19314 + 0.04 * t13565 * t14152 * t19314 - 0.008888888888888889 * t13565 * t14110 * t19314 + 0.044444444444444446 * t14162 + 0.05925925925925926 * t14170 + 0.044444444444444446 * t14181 - 0.007407407407407408 * t14183 + 0.03732469135802469 * t9552 + 0.017777777777777778 * t2060 * t3358 * t19381 + 0.013333333333333334 * t103 * t525 * t19349 - 0.0022222222222222222 * t103 * t1576 * t19354 - 0.02666666666666667 * t21216;
    (t21218,)
}
