//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 184/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk184<F: Float>(t12: F, t132: F, t479: F, t337: F, t44: F, zeta_threshold: F) -> (F, F) {
    let t13 = t12 <= zeta_threshold;
    let t481 = t132 * t479 / F::new(30.0);
    let t484 = piecewise3::<f64>(t13, F::new(0.0), F::new(2.0) * t12 * t337);
    let t485 = t484 * t44;
    (t481, t485)
}
