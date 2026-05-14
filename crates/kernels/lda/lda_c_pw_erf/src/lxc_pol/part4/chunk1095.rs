//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1095/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1095<F: Float>(t1318: F, t1401: F, t1403: F, t1466: F, t2478: F, t2550: F, t3745: F, t2549: F, t3783: F, t519: F, t10162: F, t1325: F, t2557: F, t13172: F, t6692: F, t4763: F, t5356: F) -> (F, F, F, F, F, F) {
    let t15956 = 8.0 / 15.0 * t1318 * t1466 * t1401 * t2478 * t1403;
    let t15958 = 8.0 / 45.0 * t3745 * t2550;
    let t15960 = t519 * t3783 * t2549;
    let t15961 = 8.0 / 405.0 * t15960;
    let t15963 = t1325 * t10162 * t2557;
    let t15964 = 16.0 / 135.0 * t15963;
    let t15966 = t1325 * t13172 * t6692;
    let t15967 = 8.0 / 9.0 * t15966;
    let t15969 = 8.0 / 15.0 * t4763 * t5356;
    (t15956, t15958, t15961, t15964, t15967, t15969)
}
