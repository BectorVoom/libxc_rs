//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 213/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk213(t114: f64, t655: f64, t665: f64, t653: f64, t69: f64) -> (f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t666 = t655 * t665;
    let t670 = piecewise3(t115, 0.0_f64, -t653 - t69 * t666 / 8.0_f64);
    (t666, t670)
}
