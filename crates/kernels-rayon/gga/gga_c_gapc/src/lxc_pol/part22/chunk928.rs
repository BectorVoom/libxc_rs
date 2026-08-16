//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 928/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk928(t9048: f64, t9051: f64, t9054: f64, t9057: f64, t9062: f64, t9064: f64, t9069: f64, t9073: f64, t9076: f64, t9081: f64, t9085: f64, t9088: f64, t9093: f64) -> f64 {
    let t10693 = 0.11255061864162936194e-7_f64 * t9048 + 0.11255061864162936194e-6_f64 * t9051 + 0.66704999981605668513e-8_f64 * t9054 - 0.34752370105806885418e-3_f64 * t9057 + 0.51564945349389680439e-8_f64 * t9062 - 0.9275345110817126956e-4_f64 * t9064 - 0.84540905957968605064e-6_f64 * t9069 + 0.33765185592488808582e-6_f64 * t9073 + 0.67530371184977617164e-6_f64 * t9076 + 0.33765185592488808582e-6_f64 * t9081 - 0.34752370105806885418e-3_f64 * t9085 + 0.51491428373437201896e-5_f64 * t9088 - 0.35580446990188463585e-8_f64 * t9093;
    t10693
}
