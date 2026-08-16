//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 802/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk802(t1451: f64, t2001: f64, t1434: f64, t1998: f64, t1441: f64, t1429: f64, t1418: f64, t7383: f64, t7387: f64, t7390: f64, t7396: f64, t7405: f64, t8680: f64, t8682: f64, t8684: f64, t8686: f64, t8690: f64) -> f64 {
    let t8692 = t2001 * t1451;
    let t8694 = t1998 * t1434;
    let t8696 = t2001 * t1441;
    let t8698 = t2001 * t1429;
    let t8700 = t2001 * t1418;
    let t8702 = -t7383 / 64.0_f64 - t7387 / 192.0_f64 - 0.7640625e-2_f64 * t7390 + 0.140078125e-1_f64 * t7396 + 7.0_f64 / 288.0_f64 * t7405 + 11.0_f64 / 384.0_f64 * t8680 + 11.0_f64 / 1152.0_f64 * t8682 + 7.0_f64 / 144.0_f64 * t8684 + 0.25724410870841842183e-2_f64 * t8686 - 0.10718504529517434243e-3_f64 * t8690 - 0.17149607247227894789e-2_f64 * t8692 + 0.85748036236139473944e-3_f64 * t8694 + 0.34299214494455789578e-2_f64 * t8696 + 0.85748036236139473945e-2_f64 * t8698 - 0.34299214494455789578e-2_f64 * t8700;
    t8702
}
