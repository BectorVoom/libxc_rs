//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 84/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk84<F: Float>(t242: F, t8: F, t17: F, t231: F, t237: F, t240: F, t42: F) -> (F, F, F, F, F) {
    let t243 = t8 * t242;
    let t246 = -113.69336978972719 + 6.001255378196778 * t17 + 12.335328239599177 * t231 - 5.687617677680484 * t42 + 0.17701513906783214 * t237 + 5.4944839533438375e-05 * t240 * t243;
    let t251 = -109.7426349321691 + 4.066578236106061 * t17 + 13.600858284347709 * t231 - 6.005077522251017 * t42 + 0.1875 * t237;
    let t252 = 1.0 / t251;
    let t253 = t246 * t252;
    (t243, t246, t251, t252, t253)
}
