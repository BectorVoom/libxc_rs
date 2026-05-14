//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 676/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk676<F: Float>(t504: F, t5992: F, t538: F, t503: F, t11: F, t25: F, t3530: F, t4604: F, t5093: F, t5096: F, t5112: F, t6533: F, t6536: F, t6539: F, t6542: F, t6545: F, t6547: F, t6549: F, t6551: F, t6553: F, t6555: F) -> (F, F, F, F, F) {
    let t6557 = t504 * t5992;
    let t6558 = t538 * t6557;
    let t6561 = t503 * t6557;
    let t6562 = t11 * t6561;
    let t6565 = -0.21595 * t6533 + 0.2879333333333333 * t6536 + 0.07198333333333333 * t6539 - 0.023994444444444443 * t6542 - 0.047988888888888886 * t4604 - t5093 + t5096 + t5112 - 0.023994444444444443 * t6545 + 0.011997222222222222 * t6547 + 0.007998148148148148 * t6549 + 0.0014814814814814814 * t6551 - 0.008888888888888889 * t6553 + 0.0044444444444444444 * t6555 - 0.006666666666666667 * t25 * t6558 - 0.035991666666666665 * t6562 - 0.015996296296296297 * t3530;
    (t6557, t6558, t6561, t6562, t6565)
}
