//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 936/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk936<F: Float>(t12: F, t1933: F, t2563: F, t1072: F, t19395: F, t2389: F, t337: F, t5974: F, t7300: F, t764: F, t131: F, t178: F, t44: F, t513: F, t7628: F, t6688: F, t844: F, zeta_threshold: F) -> (F, F, F, F) {
    let t13 = t12 <= zeta_threshold;
    let t19643 = t2563 * t1933;
    let t19644 = t19643 / 15.0;
    let t19654 = piecewise3(t13, 0.0, -12.0 * t1072 * t2389 + 2.0 * t12 * t19395 + 2.0 * t337 * t7300 + 6.0 * t5974 * t764);
    let t19658 = t19654 * t44 * t131 * t178 / 30.0;
    let t19660 = t7628 * t513 / 30.0;
    let t19662 = t6688 * t844 / 10.0;
    (t19644, t19658, t19660, t19662)
}
