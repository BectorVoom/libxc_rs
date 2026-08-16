//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 203/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk203(t237: f64, t846: f64, t233: f64, t235: f64, t239: f64, t820: f64, t205: f64, t242: f64) -> (f64, f64, f64, f64) {
    let t848 = 0.10003937560882938627e-2_f64 * t237 * t846;
    let t849 = t233 * t235;
    let t851 = t820 * t849 * t239;
    let t853 = 1.0_f64 / t242 / t205;
    (t848, t849, t851, t853)
}
