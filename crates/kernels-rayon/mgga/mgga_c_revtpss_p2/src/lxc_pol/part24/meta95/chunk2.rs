//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 547/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk547(t2846: f64, t340: f64, t992: f64, t338: f64) -> (f64, f64, f64) {
    let t3037 = 0.11111111111111111111e-1_f64 * t2846;
    let t3056 = 1.0_f64 / t992 / t340;
    let t3057 = t338 * t3056;
    (t3037, t3056, t3057)
}
