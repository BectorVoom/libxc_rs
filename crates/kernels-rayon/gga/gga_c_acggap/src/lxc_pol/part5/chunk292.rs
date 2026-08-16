//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 292/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk292(t1008: f64, t425: f64, t431: f64, t438: f64, t173: f64) -> (f64, f64, f64, f64, f64) {
    let t1009 = t1008 * t425;
    let t1011 = t1008 * t431;
    let t1013 = t1008 * t438;
    let t1015 = t173 * t173;
    let t1016 = 1.0_f64 / t1015;
    (t1009, t1011, t1013, t1015, t1016)
}
