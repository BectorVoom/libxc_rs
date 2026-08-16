//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 863/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk863(t8257: f64, t940: f64, t2708: f64, t2754: f64, t2761: f64, t2766: f64, t324: f64, t8226: f64, t8231: f64, t8233: f64, t8236: f64, t8241: f64, t8244: f64, t8247: f64, t8251: f64, t8254: f64) -> f64 {
    let t8258 = t940 * t8257;
    let t8260 = 0.15454509315180013964e0_f64 * t8226 + t8231 - 0.39666573908962035841e1_f64 * t8233 * t324 + 0.84999801233490076802e0_f64 * t8236 - 0.9356877183176434872e2_f64 * t2708 * t2766 + 0.1169609647897054359e2_f64 * t8241 - 0.15486228121497046737e2_f64 * t8244 + 0.4645868436449114021e2_f64 * t8247 + 0.12388982497197637389e3_f64 * t8251 * t2761 - 0.37166947491592912168e3_f64 * t8254 * t2754 - 0.389869882632351453e1_f64 * t8258;
    t8260
}
