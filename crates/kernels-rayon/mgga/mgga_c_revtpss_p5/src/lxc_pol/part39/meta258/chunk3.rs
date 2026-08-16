//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 962/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk962(t114: f64, t658: f64, t8268: f64, t69: f64, t8257: f64, t8258: f64, t8260: f64, t8264: f64, t8267: f64) -> (f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t8269 = t8268 * t658;
    let t8273 = piecewise3(t115, 0.0_f64, t8257 + t8258 * t8260 / 4.0_f64 + 5.0_f64 / 24.0_f64 * t69 * t8264 - 5.0_f64 / 24.0_f64 * t8267 * t8269);
    (t8269, t8273)
}
