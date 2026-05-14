//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 521/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk521<F: Float>(t151: F, t3233: F, t49: F, t3397: F, t3409: F, t3332: F, t3339: F, t3330: F, t3444: F, t3453: F, t169: F, t3086: F, t96: F, t839: F, t748: F, t846: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3670 = t151 * t3233;
    let t3676 = t49 * t49;
    let t3677 = 1.0 / t3676;
    let t3692 = 2.6666666666666665 * t3397;
    let t3695 = 12.0 * t3409;
    let t3696 = 1.0952258580751613 * t3332;
    let t3697 = 0.18253764301252687 * t3339;
    let t3706 = 0.821419393556371 * t3330;
    let t3713 = 12.0 * t3444;
    let t3715 = 32.0 * t3453;
    let t3727 = t96 * t169 * t3086;
    let t3729 = 0.04115066352984959 * t839 * t3727;
    let t3734 = t748 * t846;
    (t3670, t3677, t3692, t3695, t3696, t3697, t3706, t3713, t3715, t3729, t3734)
}
