//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1403/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1403(t10272: f64, t10279: f64, t10281: f64, t10288: f64, t10290: f64, t10275: f64, t10278: f64, t10284: f64, t10287: f64, t10295: f64, t13261: f64, t4171: f64, t602: f64) -> (f64, f64) {
    let t13262 = 12.0_f64 * t10272;
    let t13263 = 48.0_f64 * t10279;
    let t13264 = 80.0_f64 * t10281;
    let t13265 = 180.0_f64 * t10288;
    let t13266 = 252.0_f64 * t10290;
    let t13267 = t13261 + t13262 - t10275 - t10278 + t13263 + t13264 - t10284 - t10287 + t13265 + t13266 - t10295;
    let t13269 = t4171 * t602;
    (t13267, t13269)
}
