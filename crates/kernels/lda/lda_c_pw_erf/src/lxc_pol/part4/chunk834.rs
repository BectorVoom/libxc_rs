//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 834/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk834<F: Float>(t2479: F, t514: F, t548: F, t2120: F, t2131: F, t2504: F, t493: F, t2134: F, t795: F, t2463: F, t656: F, t2402: F, t568: F, t1976: F, t739: F, t4829: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6215 = t514 * t2479;
    let t6216 = t548 * t6215;
    let t6217 = 8.0 / 45.0 * t6216;
    let t6219 = 8.0 / 15.0 * t2120 * t2131;
    let t6220 = t514 * t2504;
    let t6221 = t493 * t6220;
    let t6222 = 8.0 / 45.0 * t6221;
    let t6223 = t795 * t2134;
    let t6224 = 8.0 / 45.0 * t6223;
    let t6225 = t2463 * t656;
    let t6227 = t2402 * t568;
    let t6228 = 8.0 / 45.0 * t6227;
    let t6229 = t1976 * t739;
    let t6230 = t4829 * t6229;
    (t6215, t6217, t6219, t6220, t6222, t6224, t6225, t6228, t6229, t6230)
}
