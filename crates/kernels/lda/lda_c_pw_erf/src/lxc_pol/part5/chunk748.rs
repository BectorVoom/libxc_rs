//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 748/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk748<F: Float>(t1349: F, t7418: F, t11: F, t557: F, t7422: F, t7426: F, t3633: F, t7408: F, t7404: F, t6638: F, t6649: F, t6657: F, t6793: F, t6795: F, t6797: F, t7433: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7434 = t1349 * t7418;
    let t7435 = t11 * t7434;
    let t7437 = t557 * t7422;
    let t7438 = t11 * t7437;
    let t7440 = t557 * t7426;
    let t7441 = t11 * t7440;
    let t7449 = t3633 * t7408;
    let t7450 = t11 * t7449;
    let t7452 = t557 * t7404;
    let t7453 = t11 * t7452;
    let t7455 = -0.07198333333333333 * t7435 - 0.21595 * t7438 + 0.21595 * t7441 + 0.013333333333333334 * t6793 + 0.0044444444444444444 * t6795 - 0.02666666666666667 * t6797 - 0.07198333333333333 * t6649 + 0.035991666666666665 * t6657 + 0.023994444444444443 * t6638 - 0.03999074074074074 * t7450 - 0.035991666666666665 * t7453;
    let t7456 = t7433 + t7455;
    (t7434, t7435, t7437, t7438, t7440, t7441, t7449, t7450, t7452, t7453, t7456)
}
