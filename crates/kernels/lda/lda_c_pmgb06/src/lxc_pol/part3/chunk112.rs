//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 112/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk112<F: Float>(t5: F, t12: F, t7: F, t9: F, t14: F, t139: F, zeta_threshold: F) -> (F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t13 = t12 <= zeta_threshold;
    let t249 = t7 * zeta_threshold;
    let t250 = t9 * t5;
    let t251 = piecewise3::<F>(t6, t249, t250);
    let t252 = t14 * t12;
    let t253 = piecewise3::<F>(t13, t249, t252);
    let t254 = t251 + t253 - F::new(2.0);
    let t257 = F::new(1.0) / (F::new(2.0) * t139 - F::new(2.0));
    (t250, t252, t254, t257)
}
