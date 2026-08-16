//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 478/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk478<F: Float>(t12: F, t1079: F, t14: F, t2386: F, t2389: F, t2385: F, t257: F, zeta_threshold: F) -> F {
    let t13 = t12 <= zeta_threshold;
    let t2393 = piecewise3::<F>(t13, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1079 * t2386 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t14 * t2389);
    let t2395 = (t2385 + t2393) * t257;
    t2395
}
