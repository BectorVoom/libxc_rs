//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1176/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1176<F: Float>(t405: F, t7853: F, t103: F, t13399: F, t13407: F, t13565: F, t14106: F, t14110: F, t14152: F, t14162: F, t14170: F, t14181: F, t14183: F, t1576: F, t17177: F, t17185: F, t17215: F, t17217: F, t17222: F, t17224: F, t19314: F, t19349: F, t19354: F, t19381: F, t2060: F, t3358: F, t525: F, t9552: F) -> F {
    let t21216 = t405 * t7853;
    let t21218 = F::cast_from(0.21595_f64) * t17177 - F::cast_from(0.02666666666666667_f64) * t17185 + F::cast_from(0.08_f64) * t17215 + F::cast_from(0.10666666666666667_f64) * t17217 - F::cast_from(0.02666666666666667_f64) * t17222 + F::cast_from(0.0044444444444444444_f64) * t17224 + F::cast_from(0.09597777777777777_f64) * t13399 + F::cast_from(0.11197407407407407_f64) * t13407 - F::cast_from(0.12_f64) * t13565 * t14106 * t19314 + F::cast_from(0.04_f64) * t13565 * t14152 * t19314 - F::cast_from(0.008888888888888889_f64) * t13565 * t14110 * t19314 + F::cast_from(0.044444444444444446_f64) * t14162 + F::cast_from(0.05925925925925926_f64) * t14170 + F::cast_from(0.044444444444444446_f64) * t14181 - F::cast_from(0.007407407407407408_f64) * t14183 + F::cast_from(0.03732469135802469_f64) * t9552 + F::cast_from(0.017777777777777778_f64) * t2060 * t3358 * t19381 + F::cast_from(0.013333333333333334_f64) * t103 * t525 * t19349 - F::cast_from(0.0022222222222222222_f64) * t103 * t1576 * t19354 - F::cast_from(0.02666666666666667_f64) * t21216;
    t21218
}
