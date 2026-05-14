//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 806/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk806<F: Float>(t51: F, t1212: F, t2471: F, t278: F, t9683: F, t9735: F, t9711: F, zeta_threshold: F) -> (F,) {
    let t52 = t51 <= zeta_threshold;
    let t9738 = piecewise3(t52, t9683, t1212 * t2471 + t278 * t9735);
    let t9739 = t9711 + t9738;
    (t9739,)
}
