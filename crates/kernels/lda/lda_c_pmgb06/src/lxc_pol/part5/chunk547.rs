//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 547/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk547<F: Float>(t247: F, t3509: F, t3510: F, t1830: F, t366: F, t349: F, t1179: F, t54: F, t55: F, t56: F, t1272: F, t2060: F, t1239: F, t361: F, t410: F, t360: F) -> (F, F, F, F, F, F, F, F) {
    let t3513 = 0.3264533333333333 * t3509 * t3510 * t247;
    let t3515 = 0.7617244444444444 * t366 * t1830;
    let t3517 = 1.5156425925925925 * t349 * t1830;
    let t3521 = 7.0 / 27.0 * t54 * t55 * t1179 * t56;
    let t3523 = 0.6529066666666666 * t1272 * t2060;
    let t3525 = 1.2991222222222223 * t1239 * t2060;
    let t3530 = t410 * t361;
    let t3531 = t360 * t3530;
    (t3513, t3515, t3517, t3521, t3523, t3525, t3530, t3531)
}
