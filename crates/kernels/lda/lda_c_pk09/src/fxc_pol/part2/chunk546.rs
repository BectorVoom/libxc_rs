//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 546/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk546<F: Float>(t131: F, t4646: F, t4645: F, t568: F, t736: F, t735: F, t197: F, t1121: F, t1124: F, t167: F, t125: F, t658: F, t151: F, t2983: F, t192: F, t3557: F) -> (F, F, F, F, F, F, F, F) {
    let t4647 = t131 * t4646;
    let t4649 = 7.108175748183851 * t4645 * t4647;
    let t4650 = t568 * t736;
    let t4652 = 6.31837844283009 * t735 * t4650;
    let t4654 = t197 * t197;
    let t4655 = 1.0 / t4654;
    let t4660 = t1124 * t1121;
    let t4667 = t167 * t167;
    let t4668 = 1.0 / t4667;
    let t4673 = t658 * t125;
    let t4684 = t151 * t2983;
    let t4689 = 2.460083242092564 * t192 * t3557;
    (t4649, t4652, t4655, t4660, t4668, t4673, t4684, t4689)
}
