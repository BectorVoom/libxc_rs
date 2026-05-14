//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 633/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk633<F: Float>(t174: F, t205: F, t3540: F, t1357: F, t325: F, t1518: F, t550: F, t548: F, t594: F, t211: F, t580: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3638 = t174 * t3540 * t205;
    let t3639 = 0.11197407407407407 * t3638;
    let t3646 = t325 * t1357;
    let t3660 = t1518 * t550;
    let t3661 = t548 * t3660;
    let t3663 = t1518 * t594;
    let t3664 = t211 * t3663;
    let t3666 = t580 * t580;
    let t3667 = 1.0 / t3666;
    (t3638, t3639, t3646, t3660, t3661, t3663, t3664, t3666, t3667)
}
