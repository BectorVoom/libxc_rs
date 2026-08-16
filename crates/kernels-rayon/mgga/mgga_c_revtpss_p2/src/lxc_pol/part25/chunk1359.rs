//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1359/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1359(t26133: f64, t571: f64, t13226: f64, t13250: f64, t1456: f64, t1458: f64, t1464: f64, t2038: f64, t2045: f64, t26094: f64, t3: f64, t4154: f64, t4168: f64, t575: f64, t7319: f64, t7337: f64, t92556: f64, t92559: f64, t92563: f64, t95119: f64, t95125: f64, t95127: f64, t95176: f64) -> f64 {
    let t95180 = t571 * t26133;
    let tv4rho3sigma0 = t3 * t575 * t95119 + t13226 * t2045 + t13250 * t2038 + 3.0_f64 * t1456 * t26133 + t1458 * t95176 + 3.0_f64 * t1464 * t26094 + 3.0_f64 * t4154 * t7337 + 3.0_f64 * t4168 * t7319 + 3.0_f64 * t92556 + 6.0_f64 * t92559 + 3.0_f64 * t92563 + 6.0_f64 * t95125 + 3.0_f64 * t95127 + 3.0_f64 * t95180;
    tv4rho3sigma0
}
