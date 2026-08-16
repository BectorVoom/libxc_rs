//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 351/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk351(t1027: f64, t1135: f64, t553: f64, t894: f64, t1110: f64, t1111: f64, t1116: f64, t1121: f64, t1125: f64, t1131: f64, t1133: f64) -> (f64, f64, f64, f64) {
    let t1136 = t1135 * t1027;
    let t1137 = t1136 * t553;
    let t1138 = t894 * t1137;
    let t1141 = t1110 + t1111 * t1116 / 288.0_f64 + 0.35500316489081544176e-1_f64 * t1121 * t1125 + t1131 + 0.18110753103726578864e-2_f64 * t1133 * t1138;
    (t1136, t1137, t1138, t1141)
}
