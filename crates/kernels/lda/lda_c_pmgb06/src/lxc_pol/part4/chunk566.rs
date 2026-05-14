//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 566/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk566<F: Float>(t12: F, t176: F, t2553: F, t166: F, t161: F, t2386: F, t2389: F, t44: F, t131: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t13 = t12 <= zeta_threshold;
    let t2554 = t2553 * t176;
    let t2555 = t166 * t2554;
    let t2557 = t161 * t2555 / 30.0;
    let t2561 = piecewise3(t13, 0.0, 2.0 * t12 * t2389 + 2.0 * t2386);
    let t2562 = t2561 * t44;
    let t2563 = t2562 * t131;
    (t2554, t2555, t2557, t2562, t2563)
}
