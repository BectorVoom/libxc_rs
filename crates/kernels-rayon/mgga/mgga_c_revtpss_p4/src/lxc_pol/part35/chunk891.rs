//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 891/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk891(t13652: f64, t13654: f64, t9415: f64, t9421: f64, t9427: f64, t9514: f64, t9517: f64, t9521: f64, t9546: f64, t9569: f64, t9574: f64, t9577: f64) -> (f64, f64, f64) {
    let t22925 = 0.51947577317044391276e2_f64 * t13652;
    let t22926 = 24.0_f64 * t13654;
    let t22927 = -t9415 + t9421 - t9427 + t9546 + t9514 - t9517 - t9521 + t9569 - t9574 - t9577 - t22925 - t22926;
    (t22925, t22926, t22927)
}
