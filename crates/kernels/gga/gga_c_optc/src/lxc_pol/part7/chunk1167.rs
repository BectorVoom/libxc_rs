//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1167/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1167<F: Float>(t24231: F, t7673: F, t23543: F, t23545: F, t23551: F, t23553: F, t23555: F, t23557: F, t23561: F, t23565: F, t23567: F, t23569: F, t23840: F, t23842: F, t23846: F, t23874: F) -> (F, F) {
    let t24233 = F::cast_from(0.20690005882282467367e4_f64) * t24231 * t7673;
    let t24248 = -F::cast_from(0.18257037037037037037e0_f64) * t23543 - F::cast_from(0.43816888888888888888e0_f64) * t23545 + F::cast_from(0.43816888888888888889e0_f64) * t23551 + F::cast_from(0.97370864197530864199e0_f64) * t23553 + F::new(0.46074375e0) * t23840 - F::new(0.28483875e1) * t23842 + F::cast_from(0.1151859375e0_f64) * t23846 + F::new(0.3071625e0) * t23874 + F::cast_from(0.10954222222222222222e1_f64) * t23555 + F::cast_from(0.13145066666666666666e1_f64) * t23557 - F::cast_from(0.98587999999999999998e0_f64) * t23561 - F::cast_from(0.82156666666666666668e-1_f64) * t23565 + F::cast_from(0.21908444444444444444e0_f64) * t23567 + F::cast_from(0.97370864197530864196e-1_f64) * t23569;
    (t24233, t24248)
}
