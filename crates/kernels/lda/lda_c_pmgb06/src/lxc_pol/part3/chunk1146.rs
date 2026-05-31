//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1146/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1146<F: Float>(t486: F, t4937: F, t161: F, t4802: F, t489: F, t4754: F, t479: F, t132: F, t137: F, t2064: F, t3058: F, t166: F, t2093: F, t3382: F) -> (F, F, F, F, F) {
    let t13684 = t486 * t4937 / F::cast_from(10.0_f64);
    let t13686 = t161 * t489 * t4802;
    let t13687 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t13686;
    let t13689 = t4754 * t479 / F::cast_from(10.0_f64);
    let t13693 = t132 * t137 * t3058 * t2064 / F::cast_from(10.0_f64);
    let t13697 = t161 * t166 * t2093 * t3382 / F::cast_from(30.0_f64);
    (t13684, t13687, t13689, t13693, t13697)
}
