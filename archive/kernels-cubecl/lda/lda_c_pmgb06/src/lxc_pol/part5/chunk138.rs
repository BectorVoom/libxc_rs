//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 138/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk138<F: Float>(t12: F, t336: F, t337: F, t335: F, zeta_threshold: F) -> F {
    let t13 = t12 <= zeta_threshold;
    let t340 = piecewise3::<F>(t13, F::cast_from(0.0_f64), F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t336 * t337);
    let t342 = t335 / F::cast_from(2.0_f64) + t340 / F::cast_from(2.0_f64);
    t342
}
