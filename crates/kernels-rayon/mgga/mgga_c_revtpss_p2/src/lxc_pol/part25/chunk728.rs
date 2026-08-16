//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 728/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk728(t7060: f64, t7064: f64, t1955: f64, t860: f64, t7056: f64) -> (f64, f64, f64) {
    let t7066 = 0.12851425765524037203e-1_f64 * t7064 * t7060;
    let t7067 = t1955 * t860;
    let t7070 = t1955 * t7056;
    (t7066, t7067, t7070)
}
