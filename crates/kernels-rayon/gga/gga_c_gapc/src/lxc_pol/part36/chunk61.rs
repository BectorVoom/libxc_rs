//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 61/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk61(t103: f64, t160: f64, t161: f64, t164: f64, t99: f64, t115: f64) -> (f64, f64) {
    let t168 = 0.619125e-2_f64 * t160 * t161 - 0.79593333333333333331e-1_f64 * t103 * t164 * t99;
    let t169 = t168 * t115;
    (t168, t169)
}
