//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 894/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk894<F: Float>(t1052: F, t7991: F, t80: F, t8315: F, t1094: F, t2362: F, t4119: F, t86: F, t8612: F, t119: F, t90: F, t9204: F) -> (F, F, F, F, F) {
    let t9443 = t1052 * t7991;
    let t9445 = t8315 * t80;
    let t9446 = t9445 * t1094;
    let t9449 = t2362 * t4119;
    let t9452 = t8612 * t86;
    let t9453 = t9452 * t119;
    let t9459 = t90 * t9204;
    (t9443, t9446, t9449, t9453, t9459)
}
