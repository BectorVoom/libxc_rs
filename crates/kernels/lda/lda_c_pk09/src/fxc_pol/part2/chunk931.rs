//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 931/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk931<F: Float>(t93: F, t9850: F, t1345: F, t9836: F, t1434: F, t2649: F, t1348: F, t1388: F, t2674: F, t747: F, t2520: F, t1481: F) -> (F, F, F, F, F, F) {
    let t9851 = t93 * t9850;
    let t9854 = t1345 * t9836;
    let t9856 = t1434 * t2649;
    let t9857 = t1348 * t9856;
    let t9860 = t1388 * t747 * t2674;
    let t9862 = t747 * t2520;
    let t9863 = t1481 * t9862;
    (t9851, t9854, t9857, t9860, t9862, t9863)
}
