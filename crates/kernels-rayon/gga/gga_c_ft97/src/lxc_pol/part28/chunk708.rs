//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 708/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk708(t13212: f64, t27073: f64, t1901: f64, t23468: f64, t23484: f64, t27199: f64, t27203: f64, t27205: f64, t27208: f64, t27212: f64, t27217: f64, t27222: f64, t27226: f64, t27229: f64, t27232: f64, t446: f64) -> f64 {
    let t27235 = t13212 * t27073;
    let t27238 = -t446 * t27199 / 3.0_f64 - t23468 / 27.0_f64 + t27203 / 9.0_f64 + t27205 / 9.0_f64 - t1901 * t27208 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t27212 - 2.0_f64 / 9.0_f64 * t1901 * t27217 + 2.0_f64 / 27.0_f64 * t1901 * t27222 + t23484 / 9.0_f64 + t27226 / 9.0_f64 - t1901 * t27229 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t27232 + 2.0_f64 / 27.0_f64 * t1901 * t27235;
    t27238
}
