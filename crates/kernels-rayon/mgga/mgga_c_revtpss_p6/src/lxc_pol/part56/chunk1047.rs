//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1047/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1047(t1413: f64, t246: f64, t31752: f64, t32192: f64, t3999: f64, t843: f64, t8583: f64, t8589: f64, t1401: f64, t1412: f64, t814: f64, t1372: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120962 = t1413 * t246;
    let t120967 = t31752 * t32192 * t1413;
    let t120975 = t8583 * t8589 * t3999 * t843;
    let t120976 = t120975 * t1401;
    let t120980 = t814 * t1412;
    let t120981 = t120980 * t1372;
    (t120962, t120967, t120975, t120976, t120980, t120981)
}
