//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 83/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk83<F: Float>(t229: F, t8: F, t92: F, t10: F, t11: F, t12: F, t129: F, t9: F, t228: F, t68: F, t1: F, t72: F) -> (F, F, F, F, F, F) {
    let t231 = t229 * t8 * t92;
    let t235 = t10 * t12 * t11;
    let t236 = t9 * t129;
    let t237 = t235 * t236;
    let t240 = t68 * t228 * t11;
    let t242 = 1.0 / t72 / t1;
    (t231, t235, t236, t237, t240, t242)
}
