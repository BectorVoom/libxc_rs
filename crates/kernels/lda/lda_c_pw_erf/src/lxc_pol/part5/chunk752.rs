//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 752/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk752<F: Float>(t6227: F, t6234: F, t6237: F, t6240: F, t6293: F, t6295: F, t6298: F, t2402: F, t835: F, t2076: F, t2480: F, t6875: F, t6301: F, t2466: F, t833: F, t3668: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7499 = 8.0 / 15.0 * t6227;
    let t7500 = 16.0 / 45.0 * t6234;
    let t7501 = 16.0 / 45.0 * t6237;
    let t7502 = 32.0 / 45.0 * t6240;
    let t7503 = 32.0 / 45.0 * t6293;
    let t7504 = 8.0 / 15.0 * t6295;
    let t7505 = 4.0 / 15.0 * t6298;
    let t7507 = 4.0 / 5.0 * t2402 * t835;
    let t7509 = 4.0 / 5.0 * t2076 * t2480;
    let t7511 = 4.0 / 5.0 * t6875 * t2480;
    let t7512 = 4.0 / 15.0 * t6301;
    let t7513 = t2466 * t833;
    let t7514 = t3668 * t7513;
    (t7499, t7500, t7501, t7502, t7503, t7504, t7505, t7507, t7509, t7511, t7512, t7513, t7514)
}
