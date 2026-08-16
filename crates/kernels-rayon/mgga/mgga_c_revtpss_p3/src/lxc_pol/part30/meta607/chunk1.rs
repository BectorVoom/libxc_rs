//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2071/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2071(t27126: f64, t7003: f64, t25856: f64, t7732: f64, t26090: f64, t7898: f64, t1353: f64, t28198: f64, t25082: f64, t28197: f64, t27833: f64, t7239: f64) -> (f64, f64, f64, f64, f64) {
    let t97647 = 4.0_f64 * t27126 * t7003;
    let t97649 = 2.0_f64 * t7732 * t25856;
    let t97653 = t7898 * t26090;
    let t97654 = t28198 * t1353;
    let t97657 = 12.0_f64 * t25082 * t28197 * t97654;
    let t97659 = 6.0_f64 * t27833 * t7239;
    (t97647, t97649, t97653, t97657, t97659)
}
