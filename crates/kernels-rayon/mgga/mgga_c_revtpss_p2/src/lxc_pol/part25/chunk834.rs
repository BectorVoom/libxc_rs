//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 834/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk834(t3994: f64, t808: f64, t9845: f64, t9278: f64, t9308: f64, t9316: f64, t9320: f64, t9325: f64, t9329: f64, t9333: f64, t9365: f64, t9374: f64, t9376: f64, t9389: f64, t9391: f64) -> (f64, f64) {
    let t9846 = t808 * t3994;
    let t9847 = t9845 * t9846;
    let t9849 = -t9278 + t9308 + t9316 + t9320 - t9325 + t9329 + t9333 + t9365 - t9374 - t9376 - t9389 - t9391;
    (t9847, t9849)
}
