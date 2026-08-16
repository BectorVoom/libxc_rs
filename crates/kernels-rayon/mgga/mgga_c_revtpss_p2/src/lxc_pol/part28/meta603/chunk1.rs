//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2083/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2083(t26090: f64, t7898: f64, t1353: f64, t28198: f64, t25082: f64, t28197: f64, t27833: f64, t7239: f64, t28177: f64, t7235: f64, t28056: f64, t4254: f64) -> (f64, f64, f64, f64, f64) {
    let t97653 = t7898 * t26090;
    let t97654 = t28198 * t1353;
    let t97657 = 12.0_f64 * t25082 * t28197 * t97654;
    let t97659 = 6.0_f64 * t27833 * t7239;
    let t97661 = 6.0_f64 * t7235 * t28177;
    let t97663 = 4.0_f64 * t4254 * t28056;
    (t97653, t97657, t97659, t97661, t97663)
}
