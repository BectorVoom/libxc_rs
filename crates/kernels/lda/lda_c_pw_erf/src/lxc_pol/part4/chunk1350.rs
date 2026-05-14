//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1350/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1350<F: Float>(t7148: F, t925: F, t7151: F, t325: F, t431: F, t7123: F, t156: F, t7154: F, t426: F, t8985: F, t8921: F, t8932: F, t8936: F, t8940: F, t8943: F, t8981: F, t8991: F, t8995: F) -> (F, F, F) {
    let t19544 = t7148 * t925;
    let t19546 = t7151 * t925;
    let t19549 = t431 * t7123 * t325;
    let t19551 = t156 * t7154;
    let t19552 = t426 * t19551;
    let t19558 = 0.3247805555555556 * t8985;
    let t19561 = 3.91744 * t19544 - 0.97936 * t19546 + 1.46904 * t19549 + t19552 / 3.0 + 3.91744 * t8921 + t8932 - t8936 + 28.0 / 27.0 * t8940 - 2.0 / 9.0 * t8943 - 0.48968 * t8981 + t19558 + 1.95872 * t8991 - 0.97936 * t8995;
    (t19551, t19558, t19561)
}
