//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 611/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk611<F: Float>(t1318: F, t5031: F, t1287: F, t1332: F, t1435: F, t10: F, t289: F, t4977: F, t293: F) -> (F, F, F) {
    let t5032 = t1318 * t5031;
    let t5033 = t5032 * t1287;
    let t5035 = t1332 * t1435;
    let t5038 = t4977 * t289 * t10;
    let t5039 = t5038 * t293;
    (t5033, t5035, t5039)
}
