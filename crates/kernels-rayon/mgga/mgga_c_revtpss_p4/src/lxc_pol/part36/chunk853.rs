//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 853/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk853(t12166: f64, t342: f64, t11631: f64, t12051: f64, t1129: f64, t3431: f64, t408: f64, t3434: f64, t421: f64, t418: f64, t240: f64, t3698: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12167 = t342 * t12166;
    let t12168 = t12051 * t11631;
    let t12226 = 1.0_f64 / t3431 / t1129;
    let t12227 = t408 * t12226;
    let t12230 = 1.0_f64 / t3434 / t421;
    let t12247 = 1.0_f64 / t3431 / t418;
    let t12248 = t408 * t12247;
    let t12254 = t240 * t3698;
    (t12167, t12168, t12227, t12230, t12248, t12254)
}
