//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 184/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk184(t451: f64, t457: f64, t459: f64, t466: f64) -> (f64, f64) {
    let t469 = 1.0_f64 + 0.86931614897887578546e-1_f64 * t457 * t459 + 0.75571056687546295932e-2_f64 * t466 * t451;
    let t470 = 1.0_f64 / t469;
    (t469, t470)
}
