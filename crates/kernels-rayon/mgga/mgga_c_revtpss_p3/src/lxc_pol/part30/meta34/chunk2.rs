//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 232/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk232(t100: f64, t658: f64, t108: f64, t101: f64, t105: f64, t656: f64, t97: f64) -> (f64, f64, f64, f64) {
    let t659 = t100 * t658;
    let t661 = -t658;
    let t662 = t108 * t661;
    let t665 = -5.0_f64 / 3.0_f64 * t656 * t101 + 5.0_f64 / 3.0_f64 * t105 * t662 + 5.0_f64 / 3.0_f64 * t97 * t659;
    (t659, t661, t662, t665)
}
