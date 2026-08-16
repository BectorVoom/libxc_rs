//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1129/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1129(t4763: f64, t6244: f64, t2011: f64, t6205: f64, t2014: f64, t2018: f64, t15579: f64, t2027: f64, t1982: f64, t2473: f64, t15943: f64, t15960: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20921 = 16.0_f64 / 5.0_f64 * t4763 * t6244;
    let t20923 = 4.0_f64 / 15.0_f64 * t6205 * t2011;
    let t20925 = 8.0_f64 / 15.0_f64 * t6205 * t2014;
    let t20927 = 4.0_f64 / 9.0_f64 * t6205 * t2018;
    let t20929 = 8.0_f64 / 15.0_f64 * t15579 * t2027;
    let t20931 = 4.0_f64 / 5.0_f64 * t1982 * t2473;
    let t20932 = 16.0_f64 / 15.0_f64 * t15943;
    let t20933 = 8.0_f64 / 135.0_f64 * t15960;
    (t20921, t20923, t20925, t20927, t20929, t20931, t20932, t20933)
}
