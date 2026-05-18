//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 532/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk532<F: Float>(t5: F, t12: F, t2192: F, t2195: F, t332: F, t395: F, t1219: F, t764: F, t1: F, t336: F, t337: F, zeta_threshold: F) -> (F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t13 = t12 <= zeta_threshold;
    let t2199 = piecewise3::<f64>(t6, F::new(0.0), -F::new(2.0) / F::new(9.0) * t2192 * t332 + F::new(4.0) / F::new(3.0) * t2195 * t395);
    let t2200 = t1219 * t764;
    let t2203 = t336 * t1;
    let t2207 = piecewise3::<f64>(t13, F::new(0.0), -F::new(2.0) / F::new(9.0) * t2200 * t337 - F::new(4.0) / F::new(3.0) * t2203 * t395);
    let t2209 = t2199 / F::new(2.0) + t2207 / F::new(2.0);
    (t2200, t2203, t2209)
}
