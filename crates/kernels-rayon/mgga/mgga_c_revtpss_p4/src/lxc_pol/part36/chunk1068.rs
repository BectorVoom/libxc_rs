//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1068/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1068(t24375: f64, t3523: f64, t16706: f64, t16876: f64, t20276: f64, t20278: f64, t20280: f64, t20283: f64, t20285: f64, t20287: f64, t24230: f64, t24234: f64, t24265: f64, t24267: f64, t24272: f64, t24275: f64) -> (f64, f64) {
    let t24376 = t24375 * t3523;
    let t24393 = -0.3883875e1_f64 * t24265 + 0.247573125e0_f64 * t24267 + 0.40256666666666666668e0_f64 * t16706 + 0.27595e0_f64 * t16876 + 0.36793333333333333333e-1_f64 * t24272 + 0.49671e0_f64 * t24275 + 0.5519e-1_f64 * t20276 - 0.33114e0_f64 * t20278 - 0.16557e0_f64 * t20280 + 0.20128333333333333333e0_f64 * t20283 - 0.60385000000000000001e0_f64 * t20285 - 0.30192500000000000001e0_f64 * t20287 + 0.33547222222222222222e0_f64 * t24230 - 0.12077e1_f64 * t24234;
    (t24376, t24393)
}
