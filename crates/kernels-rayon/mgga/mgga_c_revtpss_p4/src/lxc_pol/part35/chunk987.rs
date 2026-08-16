//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 987/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk987(t12459: f64, t12460: f64, t24238: f64, t24242: f64, t24246: f64, t24250: f64, t24289: f64, t24292: f64, t24295: f64, t24298: f64, t24313: f64, t24315: f64, t24318: f64, t24320: f64) -> f64 {
    let t24361 = 0.309885e1_f64 * t24242 + 0.516475e0_f64 * t24250 - 0.20839e0_f64 * t24289 + 0.62517e0_f64 * t24292 + 0.104195e0_f64 * t24295 - t12459 - t12460 - 0.104195e0_f64 * t24298 - 0.103295e1_f64 * t24238 + 0.309885e1_f64 * t24246 + 0.6311625e0_f64 * t24313 + 0.3529725e1_f64 * t24315 + 0.264729375e1_f64 * t24318 - 0.157790625e0_f64 * t24320;
    t24361
}
