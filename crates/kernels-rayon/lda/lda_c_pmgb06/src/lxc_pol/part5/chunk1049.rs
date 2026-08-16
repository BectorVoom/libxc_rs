//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1049/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1049(t1447: f64, t7567: f64, t7634: f64, t2466: f64, t5194: f64, t7663: f64, t1423: f64, t7542: f64, t7547: f64, t15943: f64, t15945: f64, t1894: f64, t1898: f64, t1902: f64, t6127: f64) -> f64 {
    let t19549 = t1447 * t7567;
    let t19551 = t1447 * t7634;
    let t19553 = t5194 * t2466;
    let t19555 = t1447 * t7663;
    let t19563 = t1423 * t7542;
    let t19565 = t1423 * t7547;
    let t19567 = -4.0_f64 / 45.0_f64 * t15943 - 4.0_f64 / 45.0_f64 * t15945 + 16.0_f64 / 243.0_f64 * t19549 + 2.0_f64 / 27.0_f64 * t19551 + 2.0_f64 / 45.0_f64 * t19553 + 2.0_f64 / 45.0_f64 * t19555 - t6127 * t1894 / 15.0_f64 - 2.0_f64 / 15.0_f64 * t6127 * t1898 + t6127 * t1902 / 9.0_f64 - 4.0_f64 / 45.0_f64 * t19563 + 2.0_f64 / 27.0_f64 * t19565;
    t19567
}
