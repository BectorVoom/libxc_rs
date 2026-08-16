//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 545/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk545(t658: f64, t2349: f64, t100: f64, t2256: f64, t107: f64, t661: f64, t108: f64, t101: f64, t105: f64, t2344: f64, t656: f64, t659: f64, t97: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2350 = t658 * t658;
    let t2351 = t2349 * t2350;
    let t2354 = t100 * t2256;
    let t2357 = 1.0_f64 / t107;
    let t2358 = t661 * t661;
    let t2359 = t2357 * t2358;
    let t2362 = -t2256;
    let t2363 = t108 * t2362;
    let t2366 = 40.0_f64 / 9.0_f64 * t2344 * t101 - 50.0_f64 / 9.0_f64 * t656 * t659 + 10.0_f64 / 9.0_f64 * t97 * t2351 + 5.0_f64 / 3.0_f64 * t97 * t2354 + 10.0_f64 / 9.0_f64 * t105 * t2359 + 5.0_f64 / 3.0_f64 * t105 * t2363;
    (t2350, t2357, t2358, t2359, t2362, t2363, t2366)
}
