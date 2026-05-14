//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 650/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk650<F: Float>(t1449: F, t1453: F, t519: F, t1458: F, t9: F, t1461: F, t1251: F, t187: F) -> (F, F, F, F, F, F) {
    let t3880 = t1449 * t1453;
    let t3881 = t519 * t3880;
    let t3883 = t9 * t1458;
    let t3884 = t3883 * t1461;
    let t3885 = t519 * t3884;
    let t3892 = 1.0 / t187 / t1251;
    (t3880, t3881, t3883, t3884, t3885, t3892)
}
