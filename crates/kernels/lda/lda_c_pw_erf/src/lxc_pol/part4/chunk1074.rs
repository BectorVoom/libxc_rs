//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1074/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1074<F: Float>(t2146: F, t4773: F, t11983: F, t1318: F, t2466: F, t549: F, t593: F, t1351: F, t2017: F, t2478: F, t951: F, t4777: F, t4869: F, t3416: F, t6256: F, t1319: F, t6665: F) -> (F, F, F, F, F, F, F) {
    let t15634 = 8.0 / 27.0 * t2146 * t4773;
    let t15639 = 16.0 / 5.0 * t1318 * t11983 * t2466 * t549 * t593;
    let t15644 = 8.0 / 27.0 * t1318 * t2017 * t2478 * t1351 * t951;
    let t15646 = 64.0 / 81.0 * t2146 * t4777;
    let t15648 = 32.0 / 27.0 * t2146 * t4869;
    let t15650 = 16.0 / 45.0 * t3416 * t6256;
    let t15654 = 16.0 / 45.0 * t1318 * t1319 * t6665 * t549;
    (t15634, t15639, t15644, t15646, t15648, t15650, t15654)
}
