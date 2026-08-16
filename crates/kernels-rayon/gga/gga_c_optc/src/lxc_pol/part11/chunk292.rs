//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 292/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk292(t1129: f64, t466: f64, t155: f64, t463: f64, t1132: f64) -> (f64, f64, f64) {
    let t1177 = 0.50380704458364197288e-2_f64 * t466 * t1129;
    let t1178 = t155 * t463;
    let t1179 = t1178 * t1132;
    (t1177, t1178, t1179)
}
