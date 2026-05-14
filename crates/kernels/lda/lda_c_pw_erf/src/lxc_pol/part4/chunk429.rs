//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 429/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk429<F: Float>(t119: F, t155: F, t411: F, t1657: F, t416: F, t925: F) -> (F, F, F, F) {
    let t1659 = t119 * t155 * t411;
    let t1660 = t1657 * t1659;
    let t1661 = 0.9743416666666667 * t1660;
    let t1663 = 0.6495611111111111 * t416 * t925;
    (t1659, t1660, t1661, t1663)
}
