//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 660/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk660<F: Float>(t1272: F, t2060: F, t1239: F, t361: F, t410: F, t360: F, t110: F, t1267: F, t127: F, t9: F, t14: F, t158: F) -> (F, F, F, F, F, F, F, F) {
    let t3523 = F::cast_from(0.6529066666666666_f64) * t1272 * t2060;
    let t3525 = F::cast_from(1.2991222222222223_f64) * t1239 * t2060;
    let t3530 = t410 * t361;
    let t3531 = t360 * t3530;
    let t3533 = t110 * t1267;
    let t3534 = t360 * t3533;
    let t3537 = F::new(1.0) / t9 / t127;
    let t3548 = F::new(1.0) / t14 / t158;
    (t3523, t3525, t3530, t3531, t3533, t3534, t3537, t3548)
}
