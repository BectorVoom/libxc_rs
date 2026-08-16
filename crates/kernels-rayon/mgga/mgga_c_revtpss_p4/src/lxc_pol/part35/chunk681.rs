//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 681/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk681(t7058: f64, t7407: f64, t7064: f64, t2070: f64, t2411: f64) -> (f64, f64, f64) {
    let t7409 = 0.72280234901709995518e-2_f64 * t7058 * t7407;
    let t7411 = 0.12851425765524037203e-1_f64 * t7064 * t7407;
    let t7432 = t2070 * t2411;
    (t7409, t7411, t7432)
}
