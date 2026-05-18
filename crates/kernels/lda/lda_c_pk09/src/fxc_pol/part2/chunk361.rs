//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 361/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk361<F: Float>(t1765: F, t54: F, t633: F, t285: F, t433: F) -> (F, F, F) {
    let t1766 = F::new(2.0833333333333335) * t1765;
    let t1767 = t54 * t633;
    let t1768 = t285 * t1767;
    let t1769 = t433 * t1768;
    (t1766, t1767, t1769)
}
