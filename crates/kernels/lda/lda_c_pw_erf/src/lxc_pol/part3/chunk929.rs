//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 929/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk929<F: Float>(t12254: F, t4776: F, t571: F, t2018: F, t3727: F, t1472: F, t4773: F, t4777: F, t1943: F, t2973: F, t2017: F, t1381: F, t1954: F, t4841: F, t4843: F, t2014: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12257 = 128.0 / 27.0 * t571 * t4776 * t12254;
    let t12259 = 4.0 / 9.0 * t3727 * t2018;
    let t12261 = 4.0 / 9.0 * t1472 * t4773;
    let t12263 = 32.0 / 27.0 * t1472 * t4777;
    let t12264 = t1943 * t2973;
    let t12267 = 4.0 / 27.0 * t571 * t2017 * t12264;
    let t12271 = 8.0 / 15.0 * t571 * t4841 * t1954 * t1381;
    let t12273 = 16.0 / 15.0 * t1472 * t4843;
    let t12275 = 8.0 / 15.0 * t3727 * t2014;
    (t12257, t12259, t12261, t12263, t12264, t12267, t12271, t12273, t12275)
}
