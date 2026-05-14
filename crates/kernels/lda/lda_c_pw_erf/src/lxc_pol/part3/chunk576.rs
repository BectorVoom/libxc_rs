//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 576/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk576<F: Float>(t174: F, t177: F, t3540: F, t25: F, t3508: F, t3510: F, t3512: F, t3520: F, t3524: F, t3528: F, t3530: F, t3532: F, t3534: F, t3538: F, t3507: F, t530: F) -> (F, F, F, F) {
    let t3542 = t174 * t3540 * t177;
    let t3543 = 0.11197407407407407 * t3542;
    let t3544 = -0.022222222222222223 * t3508 + 0.013333333333333334 * t3510 + 0.0044444444444444444 * t3512 - 0.002962962962962963 * t25 * t3520 - 0.006666666666666667 * t25 * t3524 - 0.035991666666666665 * t3528 - 0.047988888888888886 * t3530 + 0.035991666666666665 * t3532 + 0.023994444444444443 * t3534 - 0.03999074074074074 * t3538 - t3543;
    let t3545 = t3507 + t3544;
    let t3546 = t530 * t3545;
    (t3542, t3543, t3545, t3546)
}
