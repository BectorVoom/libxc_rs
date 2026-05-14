//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 175/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk175<F: Float>(t21: F, t583: F, t584: F, t595: F, t596: F, t600: F, t601: F, t606: F) -> (F,) {
    let t609 = -0.2071019728624174 * t583 * t584 * t21 + 0.1855079159154325 * t595 * t596 + 0.30174912456185365 * t600 * t601 - 0.29107887321813086 * t606 * t601;
    (t609,)
}
