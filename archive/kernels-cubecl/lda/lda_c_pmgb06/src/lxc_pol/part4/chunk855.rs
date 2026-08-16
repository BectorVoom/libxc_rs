//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 855/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk855<F: Float>(t12: F, t1072: F, t2200: F, t336: F, t337: F, t5966: F, t5971: F, t5974: F, t5965: F, zeta_threshold: F) -> F {
    let t13 = t12 <= zeta_threshold;
    let t5978 = piecewise3::<F>(t13, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t5966 * t337 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t2200 * t1072 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5971 * t337 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t336 * t5974);
    let t5980 = t5965 / F::cast_from(2.0_f64) + t5978 / F::cast_from(2.0_f64);
    t5980
}
