//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1041/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1041(t10272: f64, t10279: f64, t10281: f64, t10288: f64, t10290: f64, t4171: f64, t602: f64, t1466: f64, t2246: f64, t580: f64, t9342: f64, t116: f64, t4245: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13262 = 12.0_f64 * t10272;
    let t13263 = 48.0_f64 * t10279;
    let t13264 = 80.0_f64 * t10281;
    let t13265 = 180.0_f64 * t10288;
    let t13266 = 252.0_f64 * t10290;
    let t13269 = t4171 * t602;
    let t13272 = t1466 * t2246;
    let t13309 = 2.0_f64 * t580;
    let t13310 = 6.0_f64 * t9342;
    let t13426 = t4245 * t116;
    (t13262, t13263, t13264, t13265, t13266, t13269, t13272, t13309, t13310, t13426)
}
