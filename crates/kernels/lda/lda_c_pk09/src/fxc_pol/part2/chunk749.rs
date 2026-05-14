//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 749/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk749<F: Float>(t7731: F, t839: F, t164: F, t7598: F, t7602: F, t7590: F, t7578: F, t2353: F, t3836: F, t119: F, t120: F, t95: F, t61: F, t82: F, t971: F, t125: F, t8374: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8600 = t839 * t7731;
    let t8602 = t164 * t7598;
    let t8604 = t164 * t7602;
    let t8606 = t164 * t7590;
    let t8608 = t164 * t7578;
    let t8612 = t2353 * t3836;
    let t8613 = t8612 * t119;
    let t8614 = t120 * t95;
    let t8615 = t61 * t82;
    let t8616 = t8615 * t971;
    let t8617 = t8614 * t8616;
    let t8620 = t8374 * t125;
    (t8600, t8602, t8604, t8606, t8608, t8612, t8613, t8617, t8620)
}
