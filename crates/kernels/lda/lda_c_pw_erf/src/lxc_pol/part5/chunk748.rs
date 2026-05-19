//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 748/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk748<F: Float>(t589: F, t6665: F, t25: F, t3579: F, t3600: F, t3627: F, t3639: F, t4661: F, t4998: F, t5000: F, t5017: F, t6638: F, t6649: F, t6657: F, t6663: F, t6667: F, t6793: F, t6795: F, t6797: F) -> (F, F) {
    let t6802 = t589 * t6665;
    let t6810 = -t3600 - t3639 + F::cast_from(0.0044444444444444444_f64) * t6793 + F::cast_from(0.0014814814814814814_f64) * t6795 - F::cast_from(0.008888888888888889_f64) * t6797 - F::cast_from(0.023994444444444443_f64) * t6649 + F::cast_from(0.011997222222222222_f64) * t6657 + F::cast_from(0.007998148148148148_f64) * t6638 - F::cast_from(0.006666666666666667_f64) * t25 * t6802 - F::cast_from(0.035991666666666665_f64) * t6667 - F::cast_from(0.007407407407407408_f64) * t3579 - F::cast_from(0.015996296296296297_f64) * t3627 - t4998 + t5000 - F::cast_from(0.047988888888888886_f64) * t4661 + t5017 + F::cast_from(0.07198333333333333_f64) * t6663;
    (t6802, t6810)
}
