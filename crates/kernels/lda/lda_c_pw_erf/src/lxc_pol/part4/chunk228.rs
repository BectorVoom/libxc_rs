//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 228/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk228<F: Float>(t168: F, t270: F, t635: F, t247: F, t465: F, t251: F, t147: F, t19: F, t3: F) -> (F, F, F, F, F) {
    let t638 = 0.019897291109174608 * t168 * t635 * t270;
    let t639 = t465 * t247;
    let t640 = t639 * t251;
    let t643 = t147 * t19;
    let t644 = 1.0 / t3;
    (t638, t639, t640, t643, t644)
}
