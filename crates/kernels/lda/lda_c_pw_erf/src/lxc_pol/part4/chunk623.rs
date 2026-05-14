//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 623/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk623<F: Float>(t1301: F, t514: F, t493: F, t1288: F, t548: F, t1459: F, t529: F, t1283: F, t518: F) -> (F, F, F, F, F, F) {
    let t3387 = t514 * t1301;
    let t3388 = t493 * t3387;
    let t3390 = t514 * t1288;
    let t3391 = t548 * t3390;
    let t3402 = t1459 * t529;
    let t3416 = t1283 * t518;
    (t3387, t3388, t3390, t3391, t3402, t3416)
}
