//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 931/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk931<F: Float>(t19583: F, t5592: F, t156: F, t426: F, t7129: F, t7133: F, t7137: F, t431: F, t5594: F, t7102: F, t2615: F, t415: F, t5607: F, t2619: F, t443: F, t7166: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19584 = t5592 * t19583;
    let t19590 = t426 * t156 * t7129;
    let t19593 = t426 * t156 * t7133;
    let t19604 = t426 * t156 * t7137;
    let t19614 = t431 * t7102 * t5594;
    let t19645 = t415 * t2615 * t5594;
    let t19647 = t5607 * t19583;
    let t19650 = t415 * t2619 * t5594;
    let t19703 = t7166 * t443;
    (t19584, t19590, t19593, t19604, t19614, t19645, t19647, t19650, t19703)
}
