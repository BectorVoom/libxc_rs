//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 253/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk253<F: Float>(t1151: F, t272: F, t251: F, t246: F) -> (F, F, F, F) {
    let t1153 = 1.28 * t1151 * t272;
    let t1154 = t251 * t251;
    let t1155 = 1.0 / t1154;
    let t1156 = t246 * t1155;
    (t1153, t1154, t1155, t1156)
}
