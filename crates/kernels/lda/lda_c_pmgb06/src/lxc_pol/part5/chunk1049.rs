//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1049/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1049<F: Float>(t1447: F, t7567: F, t7634: F, t2466: F, t5194: F, t7663: F, t1423: F, t7542: F, t7547: F, t15943: F, t15945: F, t1894: F, t1898: F, t1902: F, t6127: F) -> F {
    let t19549 = t1447 * t7567;
    let t19551 = t1447 * t7634;
    let t19553 = t5194 * t2466;
    let t19555 = t1447 * t7663;
    let t19563 = t1423 * t7542;
    let t19565 = t1423 * t7547;
    let t19567 = -F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t15943 - F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t15945 + F::cast_from(16.0_f64) / F::cast_from(243.0_f64) * t19549 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t19551 + F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t19553 + F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t19555 - t6127 * t1894 / F::cast_from(15.0_f64) - F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t6127 * t1898 + t6127 * t1902 / F::cast_from(9.0_f64) - F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t19563 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t19565;
    t19567
}
