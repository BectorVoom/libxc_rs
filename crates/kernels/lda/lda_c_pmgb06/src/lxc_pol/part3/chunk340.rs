//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 340/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk340<F: Float>(t109: F, t342: F, t55: F, t1243: F, t349: F, t947: F, t1234: F, t38: F, t64: F, t1227: F, t56: F, t409: F, t54: F, t110: F, t361: F, t360: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1245 = t55 * t109 * t342;
    let t1246 = t1243 * t1245;
    let t1247 = 0.9743416666666667 * t1246;
    let t1249 = 0.6495611111111111 * t349 * t947;
    let t1252 = 5.84605 * t38 * t64 * t1234;
    let t1255 = 2.923025 * t38 * t56 * t1227;
    let t1259 = t54 * t55 * t409 * t56 / 9.0;
    let t1260 = t110 * t361;
    let t1261 = t360 * t1260;
    (t1245, t1246, t1247, t1249, t1252, t1255, t1259, t1260, t1261)
}
