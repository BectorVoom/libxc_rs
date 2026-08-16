//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1906/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1906(t13841: f64, t26028: f64, t5706: f64, t94429: f64, t1941: f64, t9817: f64, t48662: f64, t5651: f64, t7028: f64, t9736: f64, t13985: f64, t94423: f64) -> (f64, f64, f64, f64, f64) {
    let t98191 = t26028 * t13841;
    let t98193 = t94429 * t5706;
    let t98196 = t1941 * t9817;
    let t98197 = t98196 * t48662;
    let t98200 = t9736 * t7028 * t5651;
    let t98202 = t94423 * t13985;
    (t98191, t98193, t98197, t98200, t98202)
}
