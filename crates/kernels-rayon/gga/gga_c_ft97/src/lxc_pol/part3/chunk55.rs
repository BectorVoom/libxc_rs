//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 55/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk55(t120: f64, t126: f64, t60: f64) -> (f64, f64, f64, f64, f64) {
    let t128 = 0.1247511874e1_f64 - 0.859614445e0_f64 * t120 + 0.812904345e0_f64 * t126;
    let t129 = t128 * t128;
    let t130 = 0.56633563016285904186e-1_f64 * t60;
    let t131 = 1.0_f64 + t130;
    let t132 = t131 * t131;
    (t128, t129, t130, t131, t132)
}
