//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 312/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk312<F: Float>(t1051: F, t400: F, t1012: F, t1027: F, t1030: F) -> (F, F, F) {
    let t1052 = t400 * t1051;
    let t1053 = 0.5848223397455204 * t1052;
    let t1054 = t1027 * t1012;
    let t1055 = t1054 * t1030;
    (t1053, t1054, t1055)
}
