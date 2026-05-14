//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 774/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk774<F: Float>(t9087: F, t9097: F, t9109: F, t9124: F, t101: F, t89: F, t2305: F, t4277: F, t1062: F, t2336: F, t721: F, t1101: F, t7991: F, t4360: F, t7730: F, t1063: F) -> (F, F, F, F, F, F) {
    let t9126 = t9087 + t9097 + t9109 + t9124;
    let t9127 = t101 * t9126;
    let t9128 = t9127 * t89;
    let t9131 = t2305 * t4277;
    let t9133 = t2336 * t1062;
    let t9134 = t9133 * t721;
    let t9136 = t1101 * t7991;
    let t9150 = t4360 * t7730;
    let t9151 = t1063 * t9150;
    (t9128, t9131, t9134, t9136, t9150, t9151)
}
