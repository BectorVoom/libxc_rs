//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1373/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1373(t12361: f64, t15008: f64, t17676: f64, t17685: f64, t17686: f64, t22151: f64, t22582: f64, t22585: f64, t22588: f64, t22592: f64, t22596: f64, t22601: f64, t22604: f64, t22607: f64, t22610: f64, t22616: f64, t22619: f64, t22623: f64, t4202: f64, t5133: f64, t5459: f64, t5947: f64, t7237: f64) -> f64 {
    let t22627 = 0.26531111111111111111e0_f64 * t5133 * t22582 - 0.11791604938271604938e0_f64 * t5133 * t22585 + 0.17687407407407407407e0_f64 * t15008 * t22588 + 0.10612444444444444444e0_f64 * t5133 * t22592 - 0.88437037037037037037e-1_f64 * t5133 * t22596 - 0.1857375e-1_f64 * t4202 * t22151 - 0.15918666666666666667e0_f64 * t5133 * t22601 - 0.21224888888888888889e0_f64 * t15008 * t22604 + 0.53062222222222222222e-1_f64 * t5133 * t22607 - 0.44218518518518518518e-1_f64 * t5133 * t22610 - 0.1857375e-1_f64 * t12361 * t7237 + 0.371475e-1_f64 * t4202 * t22616 - 0.9286875e-2_f64 * t5947 * t22619 + 0.11791604938271604938e-1_f64 * t17676 + 0.9286875e-2_f64 * t22623 * t5459 - t17685 + 0.70749629629629629628e-1_f64 * t17686;
    t22627
}
