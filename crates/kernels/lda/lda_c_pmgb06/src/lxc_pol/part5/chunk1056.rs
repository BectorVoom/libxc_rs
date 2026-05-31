//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1056/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1056<F: Float>(t131: F, t178: F, t19654: F, t44: F, t513: F, t7628: F, t6688: F, t844: F, t1837: F, t2563: F, t1972: F, t6744: F) -> (F, F, F, F, F) {
    let t19658 = t19654 * t44 * t131 * t178 / F::cast_from(30.0_f64);
    let t19660 = t7628 * t513 / F::cast_from(30.0_f64);
    let t19662 = t6688 * t844 / F::cast_from(10.0_f64);
    let t19664 = t2563 * t1837 / F::cast_from(10.0_f64);
    let t19666 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1972 * t6744;
    (t19658, t19660, t19662, t19664, t19666)
}
