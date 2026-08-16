//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 107/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk107<F: Float>(t36: F, t103: F, t37: F) -> (F, F, F, F) {
    let t235 = pow_3_2::<F>(t36);
    let t238 = F::cast_from(3.79785_f64) * t37 + F::cast_from(0.8969_f64) * t36 + F::cast_from(0.204775_f64) * t235 + F::cast_from(0.123235_f64) * t103;
    let t241 = F::cast_from(1.0_f64) + F::cast_from(16.081979498692537_f64) / t238;
    let t242 = F::ln(t241);
    (t235, t238, t241, t242)
}
