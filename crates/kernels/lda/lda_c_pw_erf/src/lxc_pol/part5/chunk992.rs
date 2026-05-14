//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 992/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk992<F: Float>(t2014: F, t6205: F, t2018: F, t15579: F, t2027: F, t1982: F, t2473: F, t15943: F, t15960: F, t15963: F, t15966: F, t13202: F, t34: F, t519: F, t6426: F, t4738: F, t6469: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t20925 = 8.0 / 15.0 * t6205 * t2014;
    let t20927 = 4.0 / 9.0 * t6205 * t2018;
    let t20929 = 8.0 / 15.0 * t15579 * t2027;
    let t20931 = 4.0 / 5.0 * t1982 * t2473;
    let t20932 = 16.0 / 15.0 * t15943;
    let t20933 = 8.0 / 135.0 * t15960;
    let t20934 = 16.0 / 45.0 * t15963;
    let t20935 = 16.0 / 15.0 * t15966;
    let t20939 = 16.0 / 15.0 * t519 * t13202 * t6426 * t34;
    let t20941 = 16.0 / 15.0 * t4738 * t6469;
    (t20925, t20927, t20929, t20931, t20932, t20933, t20934, t20935, t20939, t20941)
}
