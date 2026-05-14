//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1092/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1092<F: Float>(t10474: F, t2558: F, t11946: F, t11948: F, t1472: F, t6287: F, t15614: F, t2002: F, t1310: F, t7007: F, t518: F, t6874: F, t4760: F, t1446: F, t6282: F, t558: F, t6865: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15917 = 8.0 / 15.0 * t10474 * t2558;
    let t15918 = 32.0 / 135.0 * t11946;
    let t15919 = 32.0 / 45.0 * t11948;
    let t15921 = 8.0 / 45.0 * t1472 * t6287;
    let t15923 = 32.0 / 45.0 * t15614 * t2002;
    let t15925 = 16.0 / 45.0 * t7007 * t1310;
    let t15926 = t6874 * t518;
    let t15928 = 32.0 / 45.0 * t15926 * t4760;
    let t15930 = 8.0 / 45.0 * t1446 * t6282;
    let t15931 = t6865 * t558;
    (t15917, t15918, t15919, t15921, t15923, t15925, t15926, t15928, t15930, t15931)
}
