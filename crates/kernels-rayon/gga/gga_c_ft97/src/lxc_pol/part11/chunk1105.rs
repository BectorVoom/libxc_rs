//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1105/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1105(t10896: f64, t2253: f64, t2953: f64, t8640: f64, t2941: f64, t70: f64, t9651: f64, t327: f64, t41536: f64, t2934: f64, t2920: f64, t41762: f64, t801: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43188 = t2253 * t10896;
    let t43190 = t8640 * t2953;
    let t43192 = t8640 * t2941;
    let t43194 = t70 * t9651;
    let t43195 = t327 * t41536;
    let t43200 = t8640 * t2934;
    let t43202 = t8640 * t2920;
    let t43204 = t801 * t41762;
    (t43188, t43190, t43192, t43194, t43195, t43200, t43202, t43204)
}
