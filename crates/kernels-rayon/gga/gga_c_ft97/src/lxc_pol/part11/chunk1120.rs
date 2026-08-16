//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1120/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1120(t824: f64, t9572: f64, t43468: f64, t446: f64, t10758: f64, t41454: f64, t10266: f64, t10388: f64, t193: f64, t89: f64, t295: f64, t41536: f64) -> (f64, f64, f64, f64, f64) {
    let t43469 = t9572 * t824;
    let t43471 = t446 * t43468 * t43469;
    let t43474 = t446 * t10758 * t41454;
    let t43478 = t89 * t193 * t10266 * t10388;
    let t43480 = t295 * t41536;
    (t43469, t43471, t43474, t43478, t43480)
}
