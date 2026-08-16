//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 748/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk748(t589: f64, t6665: f64, t25: f64, t3579: f64, t3600: f64, t3627: f64, t3639: f64, t4661: f64, t4998: f64, t5000: f64, t5017: f64, t6638: f64, t6649: f64, t6657: f64, t6663: f64, t6667: f64, t6793: f64, t6795: f64, t6797: f64) -> (f64, f64) {
    let t6802 = t589 * t6665;
    let t6810 = -t3600 - t3639 + 0.0044444444444444444_f64 * t6793 + 0.0014814814814814814_f64 * t6795 - 0.008888888888888889_f64 * t6797 - 0.023994444444444443_f64 * t6649 + 0.011997222222222222_f64 * t6657 + 0.007998148148148148_f64 * t6638 - 0.006666666666666667_f64 * t25 * t6802 - 0.035991666666666665_f64 * t6667 - 0.007407407407407408_f64 * t3579 - 0.015996296296296297_f64 * t3627 - t4998 + t5000 - 0.047988888888888886_f64 * t4661 + t5017 + 0.07198333333333333_f64 * t6663;
    (t6802, t6810)
}
