//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 868/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk868(t30225: f64, t431: f64, t1966: f64, t980: f64, t606: f64, t377: f64, t7636: f64, t1994: f64, t1205: f64, t7605: f64, t30193: f64, t601: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30226 = t30225 * t431;
    let t30228 = t980 * t1966;
    let t30229 = t30228 * t606;
    let t30230 = 0.28303283060643736861e-2_f64 * t30229;
    let t30231 = t377 * t7636;
    let t30232 = t30231 * t1994;
    let t30233 = 0.41930789719472202756e-2_f64 * t30232;
    let t30234 = t7605 * t1205;
    let t30238 = t30193 * t601;
    (t30226, t30228, t30230, t30231, t30233, t30234, t30238)
}
