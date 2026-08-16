//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1133/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1133(t20947: f64, t20949: f64, t20951: f64, t20953: f64, t20955: f64, t20957: f64, t20961: f64, t20963: f64, t20965: f64, t20967: f64, t20969: f64, t20971: f64, t20976: f64) -> f64 {
    let t20977 = t20947 + t20949 - t20951 - t20953 + t20955 + t20957 + t20961 + t20963 - t20965 - t20967 - t20969 + t20971 - t20976;
    t20977
}
