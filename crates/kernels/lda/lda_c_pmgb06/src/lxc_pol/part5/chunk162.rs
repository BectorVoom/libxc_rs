//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 162/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk162<F: Float>(t5: F, t418: F, t419: F, t421: F, t117: F, t123: F, t191: F, t315: F, t332: F, t44: F, t131: F, zeta_threshold: F) -> (F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t423 = F::cast_from(0.001975389032890948_f64) * t418 * t419 * t421;
    let t427 = F::cast_from(0.008980675507690957_f64) * t123 * t315 * t191 * t117;
    let t430 = piecewise3::<F>(t6, F::new(0.0), F::new(2.0) * t5 * t332);
    let t431 = t430 * t44;
    let t432 = t431 * t131;
    (t423, t427, t431, t432)
}
