//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1366/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1366(t1455: f64, t3751: f64, t5427: f64, t16069: f64, t5968: f64, t11670: f64, t538: f64, t16055: f64, t16065: f64, t12394: f64, t12397: f64, t12401: f64, t12427: f64, t15008: f64, t16034: f64, t17565: f64, t17568: f64, t17572: f64, t17575: f64, t17578: f64, t17583: f64, t17586: f64, t2429: f64, t4202: f64, t5133: f64) -> f64 {
    let t17587 = t3751 * t1455;
    let t17588 = t17587 * t5427;
    let t17591 = t5968 * t16069;
    let t17594 = t11670 * t538;
    let t17595 = t17594 * t16055;
    let t17598 = t5968 * t16065;
    let t17602 = -0.29479012345679012345e-1_f64 * t12394 + 0.17687407407407407407e-1_f64 * t12397 - t12401 + 0.53062222222222222222e-1_f64 * t2429 * t17565 - 0.15918666666666666667e0_f64 * t5133 * t17568 + 0.10612444444444444444e0_f64 * t5133 * t17572 + 0.53062222222222222222e-1_f64 * t5133 * t17575 - 0.21224888888888888888e0_f64 * t15008 * t17578 - 0.9286875e-2_f64 * t4202 * t16034 + 0.70749629629629629629e-1_f64 * t17583 - t17586 - 0.88437037037037037036e-1_f64 * t5133 * t17588 - 0.44218518518518518518e-1_f64 * t5133 * t17591 - 0.11791604938271604938e0_f64 * t5133 * t17595 + 0.17687407407407407407e0_f64 * t15008 * t17598 - 0.35374814814814814814e-1_f64 * t12427;
    t17602
}
