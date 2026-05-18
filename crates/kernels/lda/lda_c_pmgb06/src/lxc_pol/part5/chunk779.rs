//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 779/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk779<F: Float>(t5: F, t2192: F, t2381: F, t330: F, t3537: F, t7284: F, t7290: F, t2386: F, t764: F, zeta_threshold: F) -> (F, F) {
    let t6 = t5 <= zeta_threshold;
    let t7294 = piecewise3::<f64>(t6, F::new(0.0), F::new(8.0) / F::new(27.0) * t3537 * t7284 - F::new(2.0) / F::new(3.0) * t2192 * t2381 + F::new(2.0) / F::new(3.0) * t330 * t7290);
    let t7295 = t2386 * t764;
    (t7294, t7295)
}
