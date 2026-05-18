//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 619/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk619<F: Float>(t1239: F, t2060: F, t1227: F, t371: F, t361: F, t410: F, t360: F, t110: F, t1267: F, t127: F, t9: F, t1212: F, t332: F) -> (F, F, F, F, F, F, F, F) {
    let t3525 = F::new(1.2991222222222223) * t1239 * t2060;
    let t3526 = t371 * t1227;
    let t3530 = t410 * t361;
    let t3531 = t360 * t3530;
    let t3533 = t110 * t1267;
    let t3534 = t360 * t3533;
    let t3537 = F::new(1.0) / t9 / t127;
    let t3540 = t1212 * t332;
    (t3525, t3526, t3530, t3531, t3533, t3534, t3537, t3540)
}
