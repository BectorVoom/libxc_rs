//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2721/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2721(t50058: f64, t40125: f64, t40127: f64, t40132: f64, t2408: f64, t775: f64, t40139: f64, t11075: f64, t14318: f64, t14436: f64, t14468: f64, t2403: f64, t2430: f64, t262: f64, t40131: f64, t40137: f64, t4433: f64, t4541: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t50059 = 24.0_f64 * t50058;
    let t50063 = 0.18311447306006545054e-3_f64 * t40125;
    let t50064 = 0.73245789224026180215e-3_f64 * t40127;
    let t50065 = 0.17544670867903938621e1_f64 * t40132;
    let t50066 = t2408 * t775;
    let t50070 = 12.0_f64 * t40139;
    let t50078 = 18.0_f64 * t14468 * t262 * t4541 * t775 + 18.0_f64 * t11075 * t4433 * t4541 + 18.0_f64 * t14318 * t2430 * t4541 + 18.0_f64 * t14436 * t2403 * t50066 - t40131 - t40137 + t50059 - t50063 + t50064 - t50065 + t50070;
    (t50059, t50063, t50064, t50065, t50070, t50078)
}
