//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 916/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk916(t51: f64, t1212: f64, t2471: f64, t278: f64, t9683: f64, t9735: f64, t9711: f64, zeta_threshold: f64) -> f64 {
    let t52 = t51 <= zeta_threshold;
    let t9738 = piecewise3(t52, t9683, t1212 * t2471 + t278 * t9735);
    let t9739 = t9711 + t9738;
    t9739
}
