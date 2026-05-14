//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 550/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk550<F: Float>(t257: F, t72: F, t8: F, t240: F, t258: F, t4787: F, t4789: F, t4793: F, t252: F, t272: F, t1150: F, t1155: F, t1162: F, t1151: F, t1175: F, t1186: F) -> (F, F, F, F, F, F) {
    let t4796 = 1.0 / t72 / t257;
    let t4797 = t8 * t4796;
    let t4800 = 2.6672246125319017 * t4787 + 13.705920266221307 * t4789 - 11.375235355360967 * t258 + 0.5507137659888112 * t4793 + 0.00024419928681528166 * t240 * t4797;
    let t4801 = t4800 * t252;
    let t4803 = 1.28 * t4801 * t272;
    let t4804 = t1150 * t1155;
    let t4806 = 2.56 * t4804 * t1162;
    let t4807 = t1151 * t1175;
    let t4809 = t1151 * t1186;
    (t4801, t4803, t4804, t4806, t4807, t4809)
}
