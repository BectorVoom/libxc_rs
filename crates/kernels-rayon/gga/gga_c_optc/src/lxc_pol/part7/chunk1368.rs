//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1368/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1368(t1136: f64, t22046: f64, t894: f64, t1121: f64, t1124: f64, t8528: f64, t27173: f64, t9167: f64, t3107: f64, t27175: f64, t3183: f64, t8915: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27196 = t894 * t1136 * t22046;
    let t27200 = t1121 * t8528 * t1124;
    let t27202 = t9167 * t27173;
    let t27203 = t3107 * t3107;
    let t27204 = t27175 * t27203;
    let t27209 = t3183 * t27173;
    let t27210 = t27175 * t8915;
    (t27196, t27200, t27202, t27204, t27209, t27210)
}
