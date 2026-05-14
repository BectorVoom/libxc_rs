//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 351/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk351<F: Float>(t132: F, t1792: F, t93: F, t337: F, t536: F, t1747: F) -> (F, F, F, F) {
    let t1793 = t132 * t1792;
    let t1794 = t93 * t1793;
    let t1797 = t536 * t337;
    let t1798 = t1797 * t1747;
    (t1793, t1794, t1797, t1798)
}
