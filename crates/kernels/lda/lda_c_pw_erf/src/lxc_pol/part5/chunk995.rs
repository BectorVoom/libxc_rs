//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 995/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk995<F: Float>(t15926: F, t6479: F, t2018: F, t7007: F, t2526: F, t833: F, t11983: F, t571: F, t593: F, t20947: F, t20949: F, t20951: F, t20953: F, t20955: F, t20957: F, t20961: F, t20963: F, t20965: F, t20967: F) -> (F, F, F, F, F) {
    let t20969 = 16.0 / 15.0 * t15926 * t6479;
    let t20971 = 8.0 / 9.0 * t7007 * t2018;
    let t20972 = t2526 * t833;
    let t20976 = 12.0 / 5.0 * t571 * t11983 * t20972 * t593;
    let t20977 = t20947 + t20949 - t20951 - t20953 + t20955 + t20957 + t20961 + t20963 - t20965 - t20967 - t20969 + t20971 - t20976;
    (t20969, t20971, t20972, t20976, t20977)
}
