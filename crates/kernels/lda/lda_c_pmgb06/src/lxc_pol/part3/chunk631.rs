//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 631/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk631<F: Float>(t3658: F, t387: F, t1112: F, t974: F, t1039: F, t620: F, t232: F, t1025: F, t632: F, t1042: F, t241: F, t238: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3659 = t3658 * t387;
    let t3662 = t974 * t1112;
    let t3665 = F::new(1.0) / t1039 / t620;
    let t3666 = t232 * t3665;
    let t3667 = t1025 * t632;
    let t3669 = F::new(1.0) / t1042 / t241;
    let t3670 = t3667 * t3669;
    let t3672 = F::cast_from(517.260129192734_f64) * t3666 * t3670;
    let t3674 = F::new(1.0) / t1039 / t238;
    (t3659, t3662, t3665, t3666, t3667, t3669, t3670, t3672, t3674)
}
