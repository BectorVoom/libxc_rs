//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 997/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk997<F: Float>(t12064: F, t4518: F, t3667: F, t573: F, t3437: F, t822: F, t1386: F, t5215: F, t1245: F, t3966: F, t4487: F, t668: F) -> (F, F, F, F, F, F) {
    let t12065 = t12064 * t4518;
    let t12071 = t573 * t3667;
    let t12083 = t822 * t3437;
    let t12087 = t5215 * t1386;
    let t12113 = t3966 * t1245;
    let t12118 = t4487 * t668;
    (t12065, t12071, t12083, t12087, t12113, t12118)
}
