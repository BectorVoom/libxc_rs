//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1224/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1224(t13226: f64, t13250: f64, t1456: f64, t1458: f64, t1464: f64, t2111: f64, t2118: f64, t26704: f64, t26743: f64, t3: f64, t4154: f64, t4168: f64, t575: f64, t7542: f64, t7560: f64, t95182: f64, t95184: f64, t95186: f64, t95190: f64, t95196: f64, t96628: f64, t96633: f64, t96682: f64) -> f64 {
    let tv4rho3sigma1 = t3 * t575 * t96628 + t13226 * t2118 + t13250 * t2111 + 3.0_f64 * t1456 * t26743 + t1458 * t96682 + 3.0_f64 * t1464 * t26704 + 3.0_f64 * t4154 * t7560 + 3.0_f64 * t4168 * t7542 + 6.0_f64 * t95182 + 3.0_f64 * t95184 + 3.0_f64 * t95186 + 6.0_f64 * t95190 + 3.0_f64 * t95196 + 3.0_f64 * t96633;
    tv4rho3sigma1
}
