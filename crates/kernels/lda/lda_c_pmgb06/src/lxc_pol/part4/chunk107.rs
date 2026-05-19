//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 107/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk107<F: Float>(t36: F, t103: F, t37: F) -> (F, F, F, F) {
    let t235 = pow_3_2::<F>(t36);
    let t238 = F::new(3.79785) * t37 + F::new(0.8969) * t36 + F::new(0.204775) * t235 + F::new(0.123235) * t103;
    let t241 = F::new(1.0) + F::cast_from(16.081979498692537_f64) / t238;
    let t242 = F::ln(t241);
    (t235, t238, t241, t242)
}
