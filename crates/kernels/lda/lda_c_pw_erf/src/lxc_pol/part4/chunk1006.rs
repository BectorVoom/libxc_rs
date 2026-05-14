//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1006/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1006<F: Float>(t3788: F, t4738: F, t5068: F, t518: F, t558: F, t581: F, t2140: F, t3727: F, t3416: F, t5363: F, t3787: F, t4886: F, t519: F, t2171: F, t3880: F, t3884: F) -> (F, F, F, F, F, F, F, F) {
    let t12639 = t4738 * t3788;
    let t12641 = t5068 * t518;
    let t12646 = t581 * t558;
    let t12652 = t3727 * t2140;
    let t12654 = t3416 * t5363;
    let t12661 = t519 * t3787 * t4886;
    let t12665 = t2171 * t3880;
    let t12667 = t2171 * t3884;
    (t12639, t12641, t12646, t12652, t12654, t12661, t12665, t12667)
}
