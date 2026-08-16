//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 725/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk725(t1317: f64, t3853: f64, t3829: f64, t4140: f64, t5536: f64, t9278: f64, t9308: f64, t9316: f64, t9320: f64, t9325: f64, t9329: f64, t9333: f64, t9365: f64, t9374: f64, t9376: f64, t9389: f64, t9391: f64, t9394: f64) -> (f64, f64) {
    let t9395 = t1317 * t3853;
    let t9396 = 12.0_f64 * t9395;
    let t9397 = 18.0_f64 * t3829 * t4140 * t5536 - t9278 + t9308 + t9316 + t9320 - t9325 + t9329 + t9333 + t9365 - t9374 - t9376 - t9389 - t9391 + t9394 + t9396;
    (t9396, t9397)
}
