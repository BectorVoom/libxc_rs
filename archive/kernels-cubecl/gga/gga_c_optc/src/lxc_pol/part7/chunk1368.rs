//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1368/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1368<F: Float>(t1136: F, t22046: F, t894: F, t1121: F, t1124: F, t8528: F, t27173: F, t9167: F, t3107: F, t27175: F, t3183: F, t8915: F) -> (F, F, F, F, F, F) {
    let t27196 = t894 * t1136 * t22046;
    let t27200 = t1121 * t8528 * t1124;
    let t27202 = t9167 * t27173;
    let t27203 = t3107 * t3107;
    let t27204 = t27175 * t27203;
    let t27209 = t3183 * t27173;
    let t27210 = t27175 * t8915;
    (t27196, t27200, t27202, t27204, t27209, t27210)
}
