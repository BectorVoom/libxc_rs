//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 160/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk160<F: Float>(t103: F, t415: F, t325: F, t102: F, t120: F, t411: F, t118: F, t119: F, t155: F, t117: F, t4: F) -> (F, F, F, F, F) {
    let t416 = t415 * t103;
    let t418 = 0.48717083333333333 * t416 * t325;
    let t421 = 2.923025 * t102 * t120 * t411;
    let t425 = t118 * t119 * t155 * t120 / 12.0;
    let t426 = t117 * t4;
    (t416, t418, t421, t425, t426)
}
