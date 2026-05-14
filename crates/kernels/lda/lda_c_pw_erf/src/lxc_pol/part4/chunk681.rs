//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 681/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk681<F: Float>(t1314: F, t4479: F, t3965: F, t3966: F, t806: F, t1328: F, t108: F, t182: F, t267: F) -> (F, F, F, F, F, F) {
    let t4480 = t4479 * t1314;
    let t4482 = 16.0 / 45.0 * t3965 * t4480;
    let t4483 = t3966 * t806;
    let t4484 = t4483 * t1328;
    let t4486 = 16.0 / 45.0 * t3965 * t4484;
    let t4487 = t182 * t108;
    let t4488 = t4487 * t267;
    (t4480, t4482, t4484, t4486, t4487, t4488)
}
