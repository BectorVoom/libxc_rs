//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1197/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1197<F: Float>(t11670: F, t538: F, t16055: F, t16065: F, t5968: F, t12394: F, t12397: F, t12401: F, t12427: F, t15008: F, t16034: F, t17565: F, t17568: F, t17572: F, t17575: F, t17578: F, t17583: F, t17586: F, t17588: F, t17591: F, t2429: F, t4202: F, t5133: F) -> (F,) {
    let t17594 = t11670 * t538;
    let t17595 = t17594 * t16055;
    let t17598 = t5968 * t16065;
    let t17602 = -0.29479012345679012345e-1 * t12394 + 0.17687407407407407407e-1 * t12397 - t12401 + 0.53062222222222222222e-1 * t2429 * t17565 - 0.15918666666666666667e0 * t5133 * t17568 + 0.10612444444444444444e0 * t5133 * t17572 + 0.53062222222222222222e-1 * t5133 * t17575 - 0.21224888888888888888e0 * t15008 * t17578 - 0.9286875e-2 * t4202 * t16034 + 0.70749629629629629629e-1 * t17583 - t17586 - 0.88437037037037037036e-1 * t5133 * t17588 - 0.44218518518518518518e-1 * t5133 * t17591 - 0.11791604938271604938e0 * t5133 * t17595 + 0.17687407407407407407e0 * t15008 * t17598 - 0.35374814814814814814e-1 * t12427;
    (t17602,)
}
