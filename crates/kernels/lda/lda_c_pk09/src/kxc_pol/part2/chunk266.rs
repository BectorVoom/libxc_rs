//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 266/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk266<F: Float>(t1216: F, t334: F, t130: F, t747: F, t339: F, t129: F, t308: F) -> (F, F, F, F) {
    let t1217 = t1216 * t334;
    let t1219 = t747 * t130;
    let t1220 = t339 * t1219;
    let t1221 = 1.800081713982063 * t1220;
    let t1222 = t308 * t129;
    (t1217, t1220, t1221, t1222)
}
