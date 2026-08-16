//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1187/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1187(t119781: f64, t119783: f64, t126092: f64, t247: f64, t126046: f64, t837: f64, t33711: f64, t846: f64, t1568: f64, t31805: f64, t817: f64, t8485: f64) -> (f64, f64, f64, f64, f64) {
    let t126095 = t119781 * t247 * t126092 * t119783;
    let t126099 = t119781 * t247 * t126046 * t837;
    let t126108 = t33711 * t846;
    let t126110 = t31805 * t1568;
    let t126112 = t126110 * t8485 * t817;
    (t126095, t126099, t126108, t126110, t126112)
}
