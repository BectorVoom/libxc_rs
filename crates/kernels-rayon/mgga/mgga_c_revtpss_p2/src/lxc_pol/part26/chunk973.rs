//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 973/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk973(t3475: f64, t431: f64, t426: f64, t1168: f64, t3453: f64, t3479: f64, t12252: f64, t12259: f64, t12261: f64, t12263: f64, t12265: f64, t12271: f64, t12275: f64, t12279: f64, t12284: f64, t12289: f64, t12292: f64, t12323: f64, t12329: f64, t12332: f64) -> (f64, f64, f64, f64) {
    let t12428 = 1.0_f64 / t3475 / t431;
    let t12429 = t426 * t12428;
    let t12430 = t3453 * t1168;
    let t12431 = t12430 * t3479;
    let t12448 = 0.69463333333333333335e-1_f64 * t12252 + 0.46308888888888888889e-1_f64 * t12259 + 0.34731666666666666667e0_f64 * t12261 - 0.20839e0_f64 * t12263 - 0.41678000000000000001e0_f64 * t12265 - 0.20839e0_f64 * t12271 + 0.62517e0_f64 * t12275 + 0.104195e0_f64 * t12279 - 0.104195e0_f64 * t12284 + 0.62517e0_f64 * t12289 - 0.103295e1_f64 * t12292 + 0.3529725e1_f64 * t12323 + 0.264729375e1_f64 * t12329 - 0.157790625e0_f64 * t12332;
    (t12429, t12430, t12431, t12448)
}
