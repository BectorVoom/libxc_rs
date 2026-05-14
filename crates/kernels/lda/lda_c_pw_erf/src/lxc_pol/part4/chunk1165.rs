//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1165/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1165<F: Float>(t15872: F, t1991: F, t519: F, t15877: F, t5250: F, t1326: F, t15858: F, t15863: F, t1446: F, t6337: F, t15868: F, t5237: F, t6336: F, t6327: F, t1313: F, t542: F, t6557: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17140 = 4.0 / 27.0 * t519 * t1991 * t15872;
    let t17143 = 32.0 / 81.0 * t519 * t5250 * t15877;
    let t17146 = 8.0 / 45.0 * t519 * t1326 * t15858;
    let t17149 = 8.0 / 9.0 * t519 * t1991 * t15863;
    let t17151 = 8.0 / 27.0 * t1446 * t6337;
    let t17154 = 8.0 / 27.0 * t519 * t1991 * t15868;
    let t17156 = t519 * t5237 * t6336;
    let t17157 = 16.0 / 81.0 * t17156;
    let t17159 = 8.0 / 45.0 * t1446 * t6327;
    let t17163 = 8.0 / 45.0 * t519 * t1313 * t6557 * t542;
    (t17140, t17143, t17146, t17149, t17151, t17154, t17157, t17159, t17163)
}
