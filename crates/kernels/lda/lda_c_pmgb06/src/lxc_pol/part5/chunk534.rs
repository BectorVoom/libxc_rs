//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 534/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk534<F: Float>(t144: F, t3031: F, t1600: F, t511: F, t1547: F, t478: F, t132: F, t134: F, t138: F, t2897: F, t455: F, t947: F, t139: F, t1435: F, t127: F, t1437: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3032 = t144 * t3031;
    let t3038 = t511 * t1600;
    let t3055 = t1547 * t478;
    let t3056 = t132 * t3055;
    let t3080 = t138 * t2897 * t134;
    let t3081 = 0.005877407407407408 * t3080;
    let t3082 = t947 * t455;
    let t3090 = t139 * t1435;
    let t3092 = 1.0 / t1437 / t127;
    (t3032, t3038, t3055, t3056, t3080, t3081, t3082, t3090, t3092)
}
