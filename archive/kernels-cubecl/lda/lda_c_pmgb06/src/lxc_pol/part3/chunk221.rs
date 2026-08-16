//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 221/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk221<F: Float>(t107: F, t290: F, t410: F, t110: F, t242: F, t30: F, t238: F, t232: F, t27: F, t347: F, t402: F, t36: F) -> (F, F, F, F, F, F, F, F) {
    let t616 = F::cast_from(0.5694518669548363_f64) * t107 * t410 * t290;
    let t619 = F::cast_from(0.0011073470983333333_f64) * t30 * t110 * t242;
    let t620 = t238 * t238;
    let t621 = F::cast_from(1.0_f64) / t620;
    let t622 = t232 * t621;
    let t623 = t347 * t27;
    let t624 = t623 * t402;
    let t627 = F::sqrt(t36);
    (t616, t619, t620, t621, t622, t623, t624, t627)
}
