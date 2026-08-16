//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1307/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1307(t105201: f64, t22574: f64, t26162: f64, t1873: f64, t22425: f64, t652: f64, t105162: f64, t105165: f64, t105167: f64, t105169: f64, t105171: f64, t105175: f64, t105177: f64, t105179: f64, t105181: f64, t105184: f64, t105186: f64, t105188: f64, t105192: f64, t1459: f64, t1980: f64, t20698: f64, t20717: f64, t20720: f64, t28855: f64, t4028: f64, t6517: f64, t96686: f64) -> f64 {
    let t105204 = 18.0_f64 * t22574 * t26162 * t105201;
    let t105207 = 2.0_f64 * t652 * t22425 * t1873;
    let t105208 = -6.0_f64 * t1459 * t96686 + t1980 * t20698 - 6.0_f64 * t20717 * t6517 - 2.0_f64 * t20720 * t6517 - 12.0_f64 * t28855 * t4028 + t105162 + t105165 - t105167 + t105169 + t105171 + t105175 + t105177 - t105179 - t105181 - t105184 - t105186 + t105188 + t105192 + t105204 - t105207;
    t105208
}
