//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 320/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk320<F: Float>(t1070: F, t88: F, t1: F, t357: F, t397: F, t1010: F, t1012: F, t386: F) -> (F, F, F, F, F) {
    let t1071 = t1070 * t88;
    let t1072 = F::new(32.0) * t1071;
    let t1073 = t357 * t1;
    let t1074 = t1073 * t397;
    let t1075 = F::new(0.0003662311007350632) * t1074;
    let t1077 = t1010 * t1012 * t386;
    (t1072, t1073, t1074, t1075, t1077)
}
