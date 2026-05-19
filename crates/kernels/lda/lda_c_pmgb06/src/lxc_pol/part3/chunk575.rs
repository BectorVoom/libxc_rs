//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 575/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk575<F: Float>(t132: F, t3076: F, t134: F, t138: F, t2897: F, t455: F, t947: F, t1527: F, t350: F, t1533: F, t1537: F, t139: F, t1435: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3077 = t132 * t3076;
    let t3078 = t3077 / F::new(15.0);
    let t3080 = t138 * t2897 * t134;
    let t3081 = F::cast_from(0.005877407407407408_f64) * t3080;
    let t3082 = t947 * t455;
    let t3084 = t350 * t1527;
    let t3086 = t350 * t1533;
    let t3088 = t350 * t1537;
    let t3090 = t139 * t1435;
    (t3077, t3078, t3080, t3081, t3082, t3084, t3086, t3088, t3090)
}
