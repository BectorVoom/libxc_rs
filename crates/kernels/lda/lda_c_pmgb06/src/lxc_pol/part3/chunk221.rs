//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 221/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk221<F: Float>(t107: F, t290: F, t410: F, t110: F, t242: F, t30: F, t238: F, t232: F, t27: F, t347: F, t402: F, t36: F, t350: F, t405: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t616 = 0.5694518669548363 * t107 * t410 * t290;
    let t619 = 0.0011073470983333333 * t30 * t110 * t242;
    let t620 = t238 * t238;
    let t621 = 1.0 / t620;
    let t622 = t232 * t621;
    let t623 = t347 * t27;
    let t624 = t623 * t402;
    let t627 = f64::sqrt(t36);
    let t628 = t627 * t27;
    let t629 = t628 * t402;
    let t632 = -0.632975 * t624 - 0.29896666666666666 * t350 - 0.1023875 * t629 - 0.08215666666666667 * t405;
    (t616, t619, t620, t621, t622, t623, t624, t628, t629, t632)
}
