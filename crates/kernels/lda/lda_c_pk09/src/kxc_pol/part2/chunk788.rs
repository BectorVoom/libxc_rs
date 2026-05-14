//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 788/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk788<F: Float>(t1052: F, t7991: F, t80: F, t8315: F, t1094: F, t2362: F, t4119: F, t86: F, t8612: F, t119: F, t90: F, t9204: F, t1067: F, t2337: F, t1047: F, t4459: F, t4461: F, t4475: F, t8617: F, t8621: F, t8859: F, t8863: F, t8867: F, t8871: F, t98: F) -> (F,) {
    let t9443 = t1052 * t7991;
    let t9445 = t8315 * t80;
    let t9446 = t9445 * t1094;
    let t9449 = t2362 * t4119;
    let t9452 = t8612 * t86;
    let t9453 = t9452 * t119;
    let t9459 = t90 * t9204;
    let t9461 = t2337 * t1067;
    let t9467 = -t9443 / 9.0 - t9446 * t98 / 6.0 - t9449 * t98 / 6.0 - t4459 + t4461 + t9453 * t8617 / 3.0 + t8621 * t1047 / 36.0 - 0.14975624337724558 * t4475 + t9459 / 9.0 - t9461 / 9.0 - 0.01233429741534199 * t8859 + 0.01233429741534199 * t8863 + 0.01233429741534199 * t8867 - 0.14975624337724558 * t8871;
    (t9467,)
}
