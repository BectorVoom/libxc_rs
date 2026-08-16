//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 177/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk177(t11: f64, t583: f64, t22: f64, t21: f64, t3: f64) -> (f64, f64, f64, f64) {
    let t584 = t11 * t583;
    let t586 = 4.0_f64 * t584 * t22;
    let t587 = t21 * t3;
    let t588 = 1.0_f64 / t587;
    (t584, t586, t587, t588)
}
