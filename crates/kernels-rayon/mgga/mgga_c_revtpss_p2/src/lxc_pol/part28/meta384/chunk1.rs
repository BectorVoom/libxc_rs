//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1447/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1447(t13620: f64, t13622: f64, t13623: f64, t13624: f64, t13629: f64, t13631: f64, t13633: f64, t13634: f64, t13635: f64, t13636: f64, t13637: f64, t9394: f64, t9415: f64, t9421: f64, t9427: f64, t9546: f64) -> f64 {
    let t13882 = t9394 - t13620 - t13622 + t13623 - t13624 - t13629 + t13631 + t13633 - t13634 + t13635 - t9415 + t9421 + t13636 - t9427 + t13637 + t9546;
    t13882
}
