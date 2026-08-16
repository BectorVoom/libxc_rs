//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3079/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3079(t1149: f64, t16943: f64, t3384: f64, t16942: f64, t3433: f64, t3435: f64, t56262: f64, t56264: f64, t56268: f64, t56271: f64, t56275: f64, t56277: f64, t56279: f64, t56281: f64, t56283: f64) -> (f64, f64, f64) {
    let t56286 = 6.0_f64 * t3384 * t16943 * t1149;
    let t56290 = 0.48245938496077605201e2_f64 * t3433 * t16942 * t3435 * t1149;
    let t56291 = t56262 - t56264 + t56268 + t56271 + t56275 - t56277 + t56279 - t56281 + t56283 - t56286 + t56290;
    (t56286, t56290, t56291)
}
