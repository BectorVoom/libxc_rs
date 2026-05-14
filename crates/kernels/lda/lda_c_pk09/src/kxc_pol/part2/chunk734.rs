//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 734/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk734<F: Float>(t7795: F, t7797: F, t7799: F, t7801: F, t7805: F, t7809: F, t7811: F, t7814: F, t7817: F, t7834: F, t7838: F, t7842: F, t7846: F, t3317: F, t3319: F, t3335: F, t3342: F, t3384: F, t3388: F, t3393: F, t3789: F, t3792: F, t3793: F, t3794: F, t7851: F, t7855: F) -> (F, F) {
    let t8276 = -0.9074963716636242 * t7795 + 0.9074963716636242 * t7797 + 0.9074963716636242 * t7799 - 0.03016988933062603 * t7801 - 0.04525483399593904 * t7805 - 0.04525483399593904 * t7809 - 0.04525483399593904 * t7811 - 0.04525483399593904 * t7814 - 0.04525483399593904 * t7817 - 0.04525483399593904 * t7834 - 0.6806222787477182 * t7838 + 0.6806222787477182 * t7842 + 0.6806222787477182 * t7846;
    let t8286 = 0.6806222787477182 * t7851 + 0.6806222787477182 * t7855 - 0.04525483399593904 * t3335 - 0.03016988933062603 * t3342 + 1.3612445574954364 * t3384 + 1.3612445574954364 * t3388 - 1.3612445574954364 * t3393 + t3789 + t3792 + t3793 - t3794 + 0.04525483399593904 * t3317 + 0.04525483399593904 * t3319;
    (t8276, t8286)
}
