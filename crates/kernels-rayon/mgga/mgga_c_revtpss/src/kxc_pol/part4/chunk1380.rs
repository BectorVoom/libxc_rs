//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1380/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1380(t1222: f64, t12855: f64, t12910: f64, t13069: f64, t17437: f64, t17438: f64, t17444: f64, t17447: f64, t17448: f64, t17453: f64, t17456: f64, t17461: f64, t17464: f64, t17467: f64, t1797: f64, t3631: f64, t3674: f64) -> f64 {
    let t17470 = -t17437 - 0.22866142996303859718e-2_f64 * t17438 * t3674 + 0.21437009059034868486e-3_f64 * t13069 * t1797 + t17444 - t17447 - 0.28582678745379824648e-3_f64 * t17448 * t3631 - t17453 - 0.85748036236139473944e-3_f64 * t12855 * t17456 + 0.85748036236139473944e-3_f64 * t12910 * t17461 + t1222 * t17464 / 108.0_f64 + t1222 * t17467 / 216.0_f64;
    t17470
}
