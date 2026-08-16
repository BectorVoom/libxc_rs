//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1167/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1167(t24231: f64, t7673: f64, t23543: f64, t23545: f64, t23551: f64, t23553: f64, t23555: f64, t23557: f64, t23561: f64, t23565: f64, t23567: f64, t23569: f64, t23840: f64, t23842: f64, t23846: f64, t23874: f64) -> (f64, f64) {
    let t24233 = 0.20690005882282467367e4_f64 * t24231 * t7673;
    let t24248 = -0.18257037037037037037e0_f64 * t23543 - 0.43816888888888888888e0_f64 * t23545 + 0.43816888888888888889e0_f64 * t23551 + 0.97370864197530864199e0_f64 * t23553 + 0.46074375e0_f64 * t23840 - 0.28483875e1_f64 * t23842 + 0.1151859375e0_f64 * t23846 + 0.3071625e0_f64 * t23874 + 0.10954222222222222222e1_f64 * t23555 + 0.13145066666666666666e1_f64 * t23557 - 0.98587999999999999998e0_f64 * t23561 - 0.82156666666666666668e-1_f64 * t23565 + 0.21908444444444444444e0_f64 * t23567 + 0.97370864197530864196e-1_f64 * t23569;
    (t24233, t24248)
}
