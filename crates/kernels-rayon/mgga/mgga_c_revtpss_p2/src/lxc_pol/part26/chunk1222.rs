//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1222/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1222(t10192: f64, t10259: f64, t10416: f64, t118: f64, t1310: f64, t13435: f64, t1450: f64, t18163: f64, t2014: f64, t2056: f64, t2089: f64, t2093: f64, t2322: f64, t2371: f64, t2372: f64, t25082: f64, t25188: f64, t26392: f64, t26399: f64, t26405: f64, t26415: f64, t26676: f64, t28167: f64, t28286: f64, t4151: f64, t4254: f64, t46126: f64, t49560: f64, t49616: f64, t49640: f64, t49851: f64, t532: f64, t651: f64, t7235: f64, t7367: f64, t7374: f64, t7474: f64, t7484: f64, t7537: f64, t96083: f64, t96178: f64, t96231: f64, t96274: f64, t96377: f64, t96420: f64, t96466: f64, t96508: f64, t96554: f64, t96594: f64) -> f64 {
    let t96626 = -6.0_f64 * t2322 * t26415 - 6.0_f64 * t4254 * t26415 - 6.0_f64 * t651 * t7474 * t2371 - 9.0_f64 * t25082 * t26405 * t49640 - t118 * (t96083 + t96178) - 18.0_f64 * t28167 * t26405 * t49616 - 3.0_f64 * t7235 * t26392 + t2014 * t532 * (t96231 + t96274 + t96377 + t96420 + t96466 + t96508 + t96554 + t96594) * t1450 - 2.0_f64 * t651 * t2089 * t10259 - 2.0_f64 * t46126 * t2056 - 6.0_f64 * t49851 * t2056 - 6.0_f64 * t10416 * t7367 + 18.0_f64 * t25082 * t28286 * t49560 + 3.0_f64 * t25188 * t7537 + 3.0_f64 * t7484 * t4151 + t2093 * t10192 - 6.0_f64 * t26399 * t2372 - 12.0_f64 * t13435 * t7374 - 6.0_f64 * t18163 * t7374 - 6.0_f64 * t26676 * t1310;
    t96626
}
