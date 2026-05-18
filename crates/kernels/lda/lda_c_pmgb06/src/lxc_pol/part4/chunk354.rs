//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 354/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk354<F: Float>(t1263: F, t35: F, t1227: F, t64: F, t347: F, t61: F) -> (F, F, F, F) {
    let t1264 = t35 * t1263;
    let t1267 = t64 * t1227;
    let t1268 = t35 * t1267;
    let t1271 = t61 * t347;
    (t1264, t1267, t1268, t1271)
}
