//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 728/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk728(t2488: f64, t331: f64, t2491: f64, t2494: f64, t504: f64, t5992: f64, t538: f64, t503: f64, t11: f64, t25: f64, t3530: f64, t4604: f64, t5093: f64, t5096: f64, t5112: f64, t6533: f64, t6536: f64, t6539: f64, t6542: f64, t6545: f64, t6547: f64, t6549: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6551 = t331 * t2488;
    let t6553 = t331 * t2491;
    let t6555 = t331 * t2494;
    let t6557 = t504 * t5992;
    let t6558 = t538 * t6557;
    let t6561 = t503 * t6557;
    let t6562 = t11 * t6561;
    let t6565 = -0.21595_f64 * t6533 + 0.2879333333333333_f64 * t6536 + 0.07198333333333333_f64 * t6539 - 0.023994444444444443_f64 * t6542 - 0.047988888888888886_f64 * t4604 - t5093 + t5096 + t5112 - 0.023994444444444443_f64 * t6545 + 0.011997222222222222_f64 * t6547 + 0.007998148148148148_f64 * t6549 + 0.0014814814814814814_f64 * t6551 - 0.008888888888888889_f64 * t6553 + 0.0044444444444444444_f64 * t6555 - 0.006666666666666667_f64 * t25 * t6558 - 0.035991666666666665_f64 * t6562 - 0.015996296296296297_f64 * t3530;
    (t6551, t6553, t6555, t6557, t6558, t6561, t6562, t6565)
}
