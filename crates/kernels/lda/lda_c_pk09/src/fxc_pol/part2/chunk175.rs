//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 175/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk175<F: Float>(t10: F, t599: F, t30: F, t584: F, t217: F, t8: F, t21: F, t583: F, t595: F, t596: F) -> (F, F, F, F) {
    let t600 = t599 * t10;
    let t601 = t584 * t30;
    let t604 = F::new(1.0) / t217;
    let t606 = t8 * t604 * t10;
    let t609 = -F::cast_from(0.2071019728624174_f64) * t583 * t584 * t21 + F::cast_from(0.1855079159154325_f64) * t595 * t596 + F::cast_from(0.30174912456185365_f64) * t600 * t601 - F::cast_from(0.29107887321813086_f64) * t606 * t601;
    (t600, t604, t606, t609)
}
