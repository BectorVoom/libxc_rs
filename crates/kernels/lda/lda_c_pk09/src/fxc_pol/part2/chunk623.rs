//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 623/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk623<F: Float>(t6501: F, t4977: F, t55: F, t285: F, t1751: F) -> (F, F) {
    let t6502 = 3.2084841915276807 * t6501;
    let t6503 = t55 * t4977;
    let t6504 = t285 * t6503;
    let t6505 = t1751 * t6504;
    (t6502, t6505)
}
