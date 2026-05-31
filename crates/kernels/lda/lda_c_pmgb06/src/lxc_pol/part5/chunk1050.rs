//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1050/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1050<F: Float>(t11915: F, t11918: F, t15299: F, t15947: F, t176: F, t1821: F, t1826: F, t1911: F, t1912: F, t1916: F, t1920: F, t1972: F, t493: F, t5486: F, t6130: F, t6134: F, t6268: F, t6398: F, t6402: F, t6407: F, t6504: F, t6747: F) -> F {
    let t19595 = -t493 * t15947 * t1911 / F::cast_from(15.0_f64) - F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t493 * t6130 * t176 * t1826 + t493 * t15299 * t176 * t1821 / F::cast_from(9.0_f64) - t6134 * t1912 / F::cast_from(15.0_f64) - F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t6134 * t1916 + t6134 * t1920 / F::cast_from(9.0_f64) - t11915 - t11918 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1972 * t6504 + F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t6268 * t6407 + F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t493 * t5486 * t6398 + F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t493 * t6747 * t6402;
    t19595
}
