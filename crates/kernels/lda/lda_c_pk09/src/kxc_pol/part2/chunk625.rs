//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 625/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk625<F: Float>(t1758: F, t5153: F, t54: F, t72: F, t6329: F, t55: F, t1240: F, t1729: F) -> (F, F, F, F, F, F) {
    let t6510 = t1758 * t5153;
    let t6511 = t72 * t54;
    let t6515 = t6329 * t5153;
    let t6516 = t72 * t55;
    let t6517 = t1240 * t1729;
    let t6519 = t6515 * t6516 * t6517;
    (t6510, t6511, t6515, t6516, t6517, t6519)
}
