//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 376/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk376<F: Float>(t103: F, t1652: F, t933: F, t1: F, t120: F, t415: F, t119: F, t155: F, t411: F, t416: F, t925: F, t118: F, t473: F, t156: F, t427: F, t426: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1653 = t1652 * t103;
    let t1655 = 0.3247805555555556 * t1653 * t933;
    let t1657 = t415 * t120 * t1;
    let t1659 = t119 * t155 * t411;
    let t1660 = t1657 * t1659;
    let t1663 = 0.6495611111111111 * t416 * t925;
    let t1674 = t118 * t119 * t473 * t120 / 9.0;
    let t1675 = t156 * t427;
    let t1676 = t426 * t1675;
    (t1653, t1655, t1657, t1659, t1660, t1663, t1674, t1675, t1676)
}
