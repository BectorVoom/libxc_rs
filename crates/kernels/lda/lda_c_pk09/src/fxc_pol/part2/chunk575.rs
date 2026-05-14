//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 575/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk575<F: Float>(t5177: F, t1247: F, t5153: F, t5045: F, t129: F, t70: F, t284: F, t49: F) -> (F, F, F, F, F) {
    let t5178 = 2.6666666666666665 * t5177;
    let t5179 = t1247 * t5153;
    let t5187 = 0.337177226155986 * t5045;
    let t5188 = t70 * t129;
    let t5190 = t284 * t5188 * t49;
    (t5178, t5179, t5187, t5188, t5190)
}
