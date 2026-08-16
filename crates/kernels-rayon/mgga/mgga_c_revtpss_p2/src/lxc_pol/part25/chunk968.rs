//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 968/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk968(t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11147: f64, t11153: f64, t11171: f64, t11356: f64, t11359: f64, t11366: f64, t11368: f64, t11370: f64, t11373: f64, t11376: f64) -> f64 {
    let t11443 = -0.103295e1_f64 * t11138 + 0.20659e1_f64 * t11153 + 0.264729375e1_f64 * t11356 - 0.157790625e0_f64 * t11359 - 0.68863333333333333332e0_f64 * t11134 + 0.51647499999999999999e0_f64 * t11140 + 0.34431666666666666666e0_f64 * t11136 - 0.57386111111111111112e0_f64 * t11147 - 0.516475e0_f64 * t11171 - 0.34731666666666666667e0_f64 * t11366 + 0.20839e0_f64 * t11368 + 0.3529725e1_f64 * t11370 - 0.52945875e1_f64 * t11373 + 0.94674375e0_f64 * t11376;
    t11443
}
