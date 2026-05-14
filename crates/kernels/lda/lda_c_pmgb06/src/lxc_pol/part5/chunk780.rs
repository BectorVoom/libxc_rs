//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 780/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk780<F: Float>(t3501: F, t56: F, t247: F, t28: F, t342: F, t3509: F, t370: F, t366: F, t4641: F, t349: F, t1767: F, t54: F, t55: F, t1272: F, t4913: F, t1239: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8276 = t3501 * t56;
    let t8278 = t28 * t247 * t342;
    let t8279 = t8276 * t8278;
    let t8281 = t3509 * t370;
    let t8282 = t8281 * t8278;
    let t8285 = 2.5390814814814813 * t366 * t4641;
    let t8287 = 5.052141975308642 * t349 * t4641;
    let t8291 = 70.0 / 81.0 * t54 * t55 * t1767 * t56;
    let t8293 = 2.9018074074074076 * t1272 * t4913;
    let t8295 = 5.773876543209877 * t1239 * t4913;
    (t8276, t8279, t8281, t8282, t8285, t8287, t8291, t8293, t8295)
}
