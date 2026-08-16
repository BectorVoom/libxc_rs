//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1704/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1704(t26506: f64, t7064: f64, t136: f64, t2066: f64, t2457: f64) -> (f64, f64, f64) {
    let t26508 = 0.17135234354032049604e-1_f64 * t7064 * t26506;
    let t26518 = t2066 * t136;
    let t26519 = t26518 * t2457;
    (t26508, t26518, t26519)
}
