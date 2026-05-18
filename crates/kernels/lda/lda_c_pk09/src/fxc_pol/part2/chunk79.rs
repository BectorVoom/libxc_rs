//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 79/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk79<F: Float>(t44: F, t208: F, t7: F, t19: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t209 = t7 * t208;
    let t211 = zeta_threshold * zeta_threshold;
    let t212 = t44 * t44;
    let t213 = piecewise3::<f64>(t45, t211, t212);
    let t215 = f64::exp(-F::new(0.42734869200542) * t19);
    (t209, t211, t212, t213, t215)
}
