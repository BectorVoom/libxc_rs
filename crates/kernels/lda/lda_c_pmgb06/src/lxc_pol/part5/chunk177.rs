//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 177/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk177<F: Float>(t12: F, t465: F, t477: F, t137: F, t132: F, t337: F, t44: F, t131: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t13 = t12 <= zeta_threshold;
    let t478 = t465 * t477;
    let t479 = t137 * t478;
    let t481 = t132 * t479 / F::new(30.0);
    let t484 = piecewise3::<F>(t13, F::new(0.0), F::new(2.0) * t12 * t337);
    let t485 = t484 * t44;
    let t486 = t485 * t131;
    (t478, t479, t481, t485, t486)
}
