//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 975/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk975(t8278: f64, t8281: f64, t366: f64, t4641: f64, t349: f64, t1767: f64, t54: f64, t55: f64, t56: f64, t1272: f64, t4913: f64, t1239: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8282 = t8281 * t8278;
    let t8285 = 2.5390814814814813_f64 * t366 * t4641;
    let t8287 = 5.052141975308642_f64 * t349 * t4641;
    let t8291 = 70.0_f64 / 81.0_f64 * t54 * t55 * t1767 * t56;
    let t8293 = 2.9018074074074076_f64 * t1272 * t4913;
    let t8295 = 5.773876543209877_f64 * t1239 * t4913;
    (t8282, t8285, t8287, t8291, t8293, t8295)
}
