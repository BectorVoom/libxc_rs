//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 742/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk742<F: Float>(t5005: F, t5038: F, t465: F, t137: F, t132: F, t1554: F, t843: F, t161: F, t1555: F, t831: F, t1548: F, t802: F) -> (F, F, F, F, F, F, F, F) {
    let t5039 = t5005 + t5038;
    let t5040 = t465 * t5039;
    let t5041 = t137 * t5040;
    let t5043 = t132 * t5041 / F::cast_from(30.0_f64);
    let t5044 = t1554 * t843;
    let t5045 = t161 * t5044;
    let t5046 = t5045 / F::cast_from(135.0_f64);
    let t5047 = t831 * t1555;
    let t5048 = t5047 / F::cast_from(135.0_f64);
    let t5049 = t802 * t1548;
    (t5039, t5040, t5041, t5043, t5044, t5046, t5048, t5049)
}
