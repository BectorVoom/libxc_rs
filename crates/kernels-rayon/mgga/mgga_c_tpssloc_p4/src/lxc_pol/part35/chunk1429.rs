//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1429/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1429(t20162: f64, t7467: f64, t55388: f64, t7769: f64, t28893: f64, t105162: f64, t105165: f64, t105167: f64, t105169: f64, t105171: f64, t105175: f64, t105177: f64, t105179: f64, t105181: f64, t105184: f64, t105186: f64, t105188: f64, t105192: f64, t105204: f64, t105207: f64, t1774: f64, t1849: f64, t20347: f64, t20698: f64, t2165: f64, t2167: f64, t27863: f64, t29493: f64, t29497: f64, t5460: f64, t652: f64) -> (f64, f64, f64, f64) {
    let t107581 = 0.405e2_f64 * t20162 * t7467;
    let t107583 = 81.0_f64 * t55388 * t7769;
    let t107585 = 81.0_f64 * t28893 * t7467;
    let t108888 = -2.0_f64 * t20347 * t2165 * t652 - 6.0_f64 * t1774 * t29493 + 3.0_f64 * t1849 * t29497 + t20698 * t2167 - 12.0_f64 * t27863 * t5460 + t105162 + t105165 - t105167 + t105169 + t105171 + t105175 + t105177 - t105179 - t105181 - t105184 - t105186 + t105188 + t105192 + t105204 - t105207;
    (t107581, t107583, t107585, t108888)
}
