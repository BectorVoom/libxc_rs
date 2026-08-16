//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1098/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1098(t23543: f64, t23545: f64, t23551: f64, t23553: f64, t23555: f64, t23557: f64, t23561: f64, t23565: f64, t23567: f64, t23569: f64, t23576: f64, t22028: f64, t769: f64) -> (f64, f64) {
    let t23578 = -0.27366666666666666666e-2_f64 * t23543 - 0.6568e-2_f64 * t23545 + 0.6568e-2_f64 * t23551 + 0.14595555555555555556e-1_f64 * t23553 + 0.1642e-1_f64 * t23555 + 0.19704e-1_f64 * t23557 - 0.14778e-1_f64 * t23561 - 0.12315e-2_f64 * t23565 + 0.3284e-2_f64 * t23567 + 0.14595555555555555556e-2_f64 * t23569 - 0.12771111111111111111e-2_f64 * t23576;
    let t23579 = t769 * t22028;
    (t23578, t23579)
}
