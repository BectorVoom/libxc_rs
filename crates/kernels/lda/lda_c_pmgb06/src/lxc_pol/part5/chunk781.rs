//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 781/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk781<F: Float>(t12: F, t2200: F, t2389: F, t336: F, t3548: F, t7295: F, t7300: F, t7294: F, zeta_threshold: F) -> F {
    let t13 = t12 <= zeta_threshold;
    let t7304 = piecewise3::<f64>(t13, F::new(0.0), F::new(8.0) / F::new(27.0) * t3548 * t7295 - F::new(2.0) / F::new(3.0) * t2200 * t2389 + F::new(2.0) / F::new(3.0) * t336 * t7300);
    let t7306 = t7294 / F::new(2.0) + t7304 / F::new(2.0);
    t7306
}
