//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 905/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk905<F: Float>(t3834: F, t3862: F, t8740: F, t3818: F, t3859: F, t3903: F, t638: F, t643: F, t1058: F, t696: F, t965: F, t3724: F, t3758: F, t963: F, t3729: F, t971: F) -> (F, F, F, F, F, F, F) {
    let t8743 = 0.13012297560362088 * t8740 * t3834 * t3862;
    let t8746 = 1.9263893255070628 * t8740 * t3818 * t3859;
    let t8747 = t638 * t3903;
    let t8749 = t643 * t3903;
    let t8755 = 21.053605041484726 * t696 * t965 * t1058;
    let t8759 = 69.26343642272586 * t696 * t963 * t3758 * t3724;
    let t8760 = t971 * t3729;
    (t8743, t8746, t8747, t8749, t8755, t8759, t8760)
}
