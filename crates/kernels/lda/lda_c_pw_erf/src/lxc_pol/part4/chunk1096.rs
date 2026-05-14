//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1096/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1096<F: Float>(t9315: F, t1621: F, t2660: F, t3794: F, t6981: F, t529: F, t6590: F, t1325: F, t1440: F, t542: F, t12299: F, t2183: F, t3787: F, t6998: F, t12874: F, t2158: F) -> (F, F, F, F, F, F, F) {
    let t15970 = 16.0 / 405.0 * t9315;
    let t15971 = t2660 * t1621;
    let t15974 = 8.0 / 15.0 * t3794 * t6981;
    let t15975 = t529 * t6590;
    let t15979 = 8.0 / 15.0 * t1325 * t1440 * t15975 * t542;
    let t15981 = 16.0 / 15.0 * t12299 * t2183;
    let t15983 = t1325 * t3787 * t6998;
    let t15984 = 16.0 / 45.0 * t15983;
    let t15986 = 16.0 / 15.0 * t12874 * t2158;
    (t15970, t15971, t15974, t15979, t15981, t15984, t15986)
}
