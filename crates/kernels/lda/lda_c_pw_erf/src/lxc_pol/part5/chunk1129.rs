//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1129/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1129<F: Float>(t4763: F, t6244: F, t2011: F, t6205: F, t2014: F, t2018: F, t15579: F, t2027: F, t1982: F, t2473: F, t15943: F, t15960: F) -> (F, F, F, F, F, F, F, F) {
    let t20921 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t4763 * t6244;
    let t20923 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t6205 * t2011;
    let t20925 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t6205 * t2014;
    let t20927 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t6205 * t2018;
    let t20929 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t15579 * t2027;
    let t20931 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t1982 * t2473;
    let t20932 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t15943;
    let t20933 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t15960;
    (t20921, t20923, t20925, t20927, t20929, t20931, t20932, t20933)
}
