//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 584/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk584<F: Float>(t2973: F, t4364: F, t1098: F, t132: F, t409: F, t1063: F, t3290: F, t1076: F, t3230: F, t3233: F, t1067: F, t1095: F) -> (F, F, F, F, F, F, F) {
    let t4365 = t4364 * t2973;
    let t4366 = t1098 * t4365;
    let t4368 = t409 * t132;
    let t4379 = t1063 * t3290 / F::new(6.0);
    let t4380 = t1076 * t3230;
    let t4382 = t1076 * t3233;
    let t4384 = t1095 * t1067;
    (t4365, t4366, t4368, t4379, t4380, t4382, t4384)
}
