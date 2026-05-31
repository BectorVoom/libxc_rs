//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1132/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1132<F: Float>(t15582: F, t2035: F, t2011: F, t7007: F, t2014: F, t15926: F, t6479: F, t2018: F, t2526: F, t833: F, t11983: F, t571: F, t593: F) -> (F, F, F, F, F, F, F) {
    let t20963 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t15582 * t2035;
    let t20965 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t7007 * t2011;
    let t20967 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t7007 * t2014;
    let t20969 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t15926 * t6479;
    let t20971 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t7007 * t2018;
    let t20972 = t2526 * t833;
    let t20976 = F::cast_from(12.0_f64) / F::cast_from(5.0_f64) * t571 * t11983 * t20972 * t593;
    (t20963, t20965, t20967, t20969, t20971, t20972, t20976)
}
