//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 931/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk931<F: Float>(t2466: F, t5194: F, t1447: F, t7663: F, t1423: F, t7542: F, t7547: F, t15943: F, t15945: F, t1894: F, t1898: F, t1902: F, t19549: F, t19551: F, t6127: F, t11915: F, t11918: F, t15299: F, t15947: F, t176: F, t1821: F, t1826: F, t1911: F, t1912: F, t1916: F, t1920: F, t1972: F, t493: F, t5486: F, t6130: F, t6134: F, t6268: F, t6398: F, t6402: F, t6407: F, t6504: F, t6747: F) -> (F, F) {
    let t19553 = t5194 * t2466;
    let t19555 = t1447 * t7663;
    let t19563 = t1423 * t7542;
    let t19565 = t1423 * t7547;
    let t19567 = -4.0 / 45.0 * t15943 - 4.0 / 45.0 * t15945 + 16.0 / 243.0 * t19549 + 2.0 / 27.0 * t19551 + 2.0 / 45.0 * t19553 + 2.0 / 45.0 * t19555 - t6127 * t1894 / 15.0 - 2.0 / 15.0 * t6127 * t1898 + t6127 * t1902 / 9.0 - 4.0 / 45.0 * t19563 + 2.0 / 27.0 * t19565;
    let t19595 = -t493 * t15947 * t1911 / 15.0 - 2.0 / 15.0 * t493 * t6130 * t176 * t1826 + t493 * t15299 * t176 * t1821 / 9.0 - t6134 * t1912 / 15.0 - 2.0 / 15.0 * t6134 * t1916 + t6134 * t1920 / 9.0 - t11915 - t11918 - 2.0 / 3.0 * t1972 * t6504 + 8.0 / 15.0 * t6268 * t6407 + 2.0 / 15.0 * t493 * t5486 * t6398 + 2.0 / 5.0 * t493 * t6747 * t6402;
    (t19567, t19595)
}
