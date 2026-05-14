//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1069/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1069<F: Float>(t7724: F, t778: F, t800: F, t2410: F, t7624: F, t2415: F, t2449: F, t2419: F, t2378: F, t7664: F, t7668: F, t774: F, t7673: F, t23543: F, t23545: F, t23551: F, t23553: F, t23555: F, t23557: F, t23561: F, t23565: F, t23567: F, t23569: F, t23840: F, t23842: F, t23846: F, t23874: F) -> (F, F, F, F, F, F) {
    let t24221 = t7724 * t778;
    let t24223 = 4.0 * t24221 * t800;
    let t24225 = 6.0 * t7624 * t2410;
    let t24226 = t2449 * t2415;
    let t24228 = 0.96490945932906628932e2 * t24226 * t2419;
    let t24230 = 4.0 * t2378 * t7664;
    let t24231 = t774 * t7668;
    let t24233 = 0.20690005882282467367e4 * t24231 * t7673;
    let t24248 = -0.18257037037037037037e0 * t23543 - 0.43816888888888888888e0 * t23545 + 0.43816888888888888889e0 * t23551 + 0.97370864197530864199e0 * t23553 + 0.46074375e0 * t23840 - 0.28483875e1 * t23842 + 0.1151859375e0 * t23846 + 0.3071625e0 * t23874 + 0.10954222222222222222e1 * t23555 + 0.13145066666666666666e1 * t23557 - 0.98587999999999999998e0 * t23561 - 0.82156666666666666668e-1 * t23565 + 0.21908444444444444444e0 * t23567 + 0.97370864197530864196e-1 * t23569;
    (t24223, t24225, t24228, t24230, t24233, t24248)
}
