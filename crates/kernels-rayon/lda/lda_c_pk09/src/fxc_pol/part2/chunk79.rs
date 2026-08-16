//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 79/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk79(t44: f64, t208: f64, t7: f64, t19: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t45 = t44 <= zeta_threshold;
    let t209 = t7 * t208;
    let t211 = zeta_threshold * zeta_threshold;
    let t212 = t44 * t44;
    let t213 = piecewise3(t45, t211, t212);
    let t215 = f64::exp(-0.42734869200542_f64 * t19);
    (t209, t211, t212, t213, t215)
}
