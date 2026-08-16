//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 38/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk38(t55: f64, t44: f64, t52: f64, t54: f64, t41: f64, t42: f64, t47: f64, rho0: f64) -> (f64, f64, f64, f64) {
    let t56 = t55 * rho0;
    let t58 = 1.0_f64 / t44 / t56;
    let t60 = t52 * t54 * t58;
    let t61 = 0.55569193573523559258e-3_f64 * t60;
    let t62 = 1.0_f64 + 0.45058854638888888889e-1_f64 * t41 * t42 * t47 + t61;
    (t56, t60, t61, t62)
}
