//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 619/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk619<F: Float>(t1322: F, t5081: F, t1477: F, t747: F, t1513: F, t1216: F, t1223: F, t1311: F, t4979: F, t1314: F, t5031: F, t1287: F) -> (F, F, F, F, F, F) {
    let t5082 = t1322 * t5081;
    let t5084 = t747 * t1477;
    let t5085 = t1513 * t5084;
    let t5087 = t1216 * t1223;
    let t5090 = F::new(1.8805371096875316) * t1311 * t4979;
    let t5091 = t1314 * t5031;
    let t5092 = t5091 * t1287;
    (t5082, t5084, t5085, t5087, t5090, t5092)
}
