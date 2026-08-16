//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 972/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk972(t34421: f64, t1988: f64, t8536: f64, t2278: f64, t7600: f64, t2290: f64, t7610: f64, t30374: f64, t8477: f64, t1181: f64, t4555: f64, t599: f64, t7493: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34422 = 7.0_f64 / 144.0_f64 * t34421;
    let t34429 = t1988 * t8536;
    let t34430 = 0.10718504529517434243e-2_f64 * t34429;
    let t34433 = t7600 * t2278;
    let t34435 = t7610 * t2290;
    let t34449 = t30374 * t8477;
    let t34453 = t7493 * t1181 * t599 * t4555;
    (t34422, t34430, t34433, t34435, t34449, t34453)
}
