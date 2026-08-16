//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 141/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk141(t406: f64, t409: f64, t412: f64, t416: f64) -> (f64, f64, f64, f64) {
    let t431 = 0.705945e1_f64 * t409 + 0.1549425e1_f64 * t406 + 0.420775e0_f64 * t412 + 0.1562925e0_f64 * t416;
    let t434 = 1.0_f64 + 0.32163958997385070134e2_f64 / t431;
    let t435 = f64::ln(t434);
    let t439 = 1.0_f64 + 0.278125e-1_f64 * t406;
    (t431, t434, t435, t439)
}
