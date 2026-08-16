//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1052/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1052(t26255: f64, t8950: f64, t2855: f64, t3107: f64, t140: f64, t24563: f64, t446: f64, t464: f64, t2849: f64, t381: f64, t26336: f64, t9167: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27129 = t8950 * t26255;
    let t27152 = t3107 * t2855;
    let t27173 = t446 * t24563 * t140;
    let t27174 = t464 * t27173;
    let t27188 = 1.0_f64 / t381 / t2849;
    let t27189 = t27188 * t26336;
    let t27202 = t9167 * t27173;
    (t27129, t27152, t27173, t27174, t27189, t27202)
}
