//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 547/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk547<F: Float>(t221: F, t2402: F, t784: F, t181: F, t184: F) -> (F, F, F, F) {
    let t2404 = 4.0 / 15.0 * t2402 * t221;
    let t2405 = t784 * t784;
    let t2406 = t2405 * t181;
    let t2407 = t2406 * t184;
    (t2404, t2405, t2406, t2407)
}
