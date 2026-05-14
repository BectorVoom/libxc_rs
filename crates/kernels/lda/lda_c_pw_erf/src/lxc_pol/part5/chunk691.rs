//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 691/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk691<F: Float>(t5176: F, t4029: F, t2405: F, t509: F, t184: F, t199: F, t2407: F, t515: F, t2523: F, t331: F, t2517: F, t2520: F, t589: F, t6665: F, t25: F, t3579: F, t3600: F, t3627: F, t3639: F, t4661: F, t4998: F, t5000: F, t5017: F, t6638: F, t6649: F, t6657: F, t6663: F, t6667: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6785 = 16.0 / 45.0 * t5176;
    let t6786 = 4.0 / 135.0 * t4029;
    let t6787 = t2405 * t509;
    let t6788 = t6787 * t184;
    let t6790 = 4.0 / 15.0 * t6788 * t199;
    let t6791 = t2407 * t515;
    let t6792 = 8.0 / 45.0 * t6791;
    let t6793 = t331 * t2523;
    let t6795 = t331 * t2517;
    let t6797 = t331 * t2520;
    let t6802 = t589 * t6665;
    let t6810 = -t3600 - t3639 + 0.0044444444444444444 * t6793 + 0.0014814814814814814 * t6795 - 0.008888888888888889 * t6797 - 0.023994444444444443 * t6649 + 0.011997222222222222 * t6657 + 0.007998148148148148 * t6638 - 0.006666666666666667 * t25 * t6802 - 0.035991666666666665 * t6667 - 0.007407407407407408 * t3579 - 0.015996296296296297 * t3627 - t4998 + t5000 - 0.047988888888888886 * t4661 + t5017 + 0.07198333333333333 * t6663;
    (t6785, t6786, t6787, t6788, t6790, t6791, t6792, t6793, t6795, t6797, t6802, t6810)
}
