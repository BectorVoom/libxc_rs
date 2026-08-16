//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3477/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3477(t65388: f64, t65389: f64, t65391: f64, t65392: f64, t65395: f64, t65396: f64, t65398: f64, t65422: f64, t19658: f64, t3169: f64, t13312: f64, t1469: f64) -> (f64, f64, f64) {
    let t65425 = t65388 + t65389 + t65391 + t65392 + t65395 + t65396 + t65398 + t65422;
    let t65431 = t3169 * t19658;
    let t65433 = t1469 * t13312;
    (t65425, t65431, t65433)
}
