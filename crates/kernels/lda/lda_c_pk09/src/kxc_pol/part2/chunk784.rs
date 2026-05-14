//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 784/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk784<F: Float>(t7795: F, t7797: F, t7799: F, t7801: F, t7805: F, t7809: F, t7811: F, t7814: F, t7817: F, t7834: F, t7838: F, t7842: F, t7846: F, t3317: F, t3319: F, t3335: F, t3342: F, t3384: F, t3388: F, t3393: F, t4231: F, t4234: F, t4235: F, t4236: F, t7851: F, t7855: F) -> (F, F) {
    let t9353 = -6.129211220482733 * t7795 + 6.129211220482733 * t7797 + 6.129211220482733 * t7799 - 0.2037667917801196 * t7801 - 0.3056501876701794 * t7805 - 0.3056501876701794 * t7809 - 0.3056501876701794 * t7811 - 0.3056501876701794 * t7814 - 0.3056501876701794 * t7817 - 0.3056501876701794 * t7834 - 4.59690841536205 * t7838 + 4.59690841536205 * t7842 + 4.59690841536205 * t7846;
    let t9363 = 4.59690841536205 * t7851 + 4.59690841536205 * t7855 - 0.3056501876701794 * t3335 - 0.2037667917801196 * t3342 + 9.1938168307241 * t3384 + 9.1938168307241 * t3388 - 9.1938168307241 * t3393 + t4231 + t4234 + t4235 - t4236 + 0.3056501876701794 * t3317 + 0.3056501876701794 * t3319;
    (t9353, t9363)
}
