//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 609/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk609<F: Float>(t6277: F, t633: F, t93: F, t1836: F, t1781: F, t1841: F, t1729: F, t68: F) -> (F, F, F) {
    let t6278 = t6277 * t633;
    let t6279 = t93 * t6278;
    let t6280 = t1836 * t6279;
    let t6282 = t1781 * t1841;
    let t6287 = t1729 * t68;
    (t6280, t6282, t6287)
}
