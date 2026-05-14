//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 629/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk629<F: Float>(t1124: F, t56: F, t174: F, t177: F, t1518: F, t495: F, t493: F, t543: F, t185: F, t1279: F, t514: F, t1294: F, t565: F, t1382: F, t211: F, t590: F, t933: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3540 = t1124 * t56;
    let t3542 = t174 * t3540 * t177;
    let t3543 = 0.11197407407407407 * t3542;
    let t3550 = t1518 * t495;
    let t3551 = t493 * t3550;
    let t3553 = t1518 * t543;
    let t3554 = t185 * t3553;
    let t3556 = t514 * t1279;
    let t3557 = t185 * t3556;
    let t3570 = t565 * t1294;
    let t3576 = t514 * t1382;
    let t3577 = t211 * t3576;
    let t3579 = t933 * t590;
    (t3540, t3542, t3543, t3550, t3551, t3553, t3554, t3556, t3557, t3570, t3576, t3577, t3579)
}
