//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 998/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk998<F: Float>(t12118: F, t4502: F, t3675: F, t521: F, t4497: F, t108: F, t2119: F, t267: F, t2103: F, t10467: F, t2030: F, t519: F, t3860: F, t4738: F, t1450: F, t5327: F) -> (F, F, F, F, F, F, F, F) {
    let t12119 = t12118 * t4502;
    let t12121 = t521 * t3675;
    let t12129 = t12118 * t4497;
    let t12136 = t2119 * t108 * t267;
    let t12143 = t2103 * t108 * t267;
    let t12196 = t519 * t10467 * t2030;
    let t12251 = t4738 * t3860;
    let t12297 = t5327 * t1450;
    (t12119, t12121, t12129, t12136, t12143, t12196, t12251, t12297)
}
