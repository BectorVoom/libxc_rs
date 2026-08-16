//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 841/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk841(t41548: f64, t793: f64, t1347: f64, t2408: f64, t118: f64, t2001: f64, t352: f64, t38523: f64, t34884: f64, t9118: f64, t2283: f64, t34881: f64) -> (f64, f64, f64, f64, f64) {
    let t41549 = t793 * t41548;
    let t41550 = 0.15965655602485078085e0_f64 * t41549;
    let t41571 = t1347 * t2408;
    let t41576 = t2001 * t118 * t38523 * t352;
    let t41579 = t34884 * t9118;
    let t41581 = t34881 * t2283;
    (t41550, t41571, t41576, t41579, t41581)
}
