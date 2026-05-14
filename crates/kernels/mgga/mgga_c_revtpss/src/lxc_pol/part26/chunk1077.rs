//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1077/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1077<F: Float>(t10192: F, t10259: F, t10416: F, t118: F, t1310: F, t13435: F, t1450: F, t18163: F, t2014: F, t2056: F, t2089: F, t2093: F, t2322: F, t2371: F, t2372: F, t25082: F, t25188: F, t26392: F, t26399: F, t26405: F, t26415: F, t26676: F, t28167: F, t28286: F, t4151: F, t4254: F, t46126: F, t49560: F, t49616: F, t49640: F, t49851: F, t532: F, t651: F, t7235: F, t7367: F, t7374: F, t7474: F, t7484: F, t7537: F, t96083: F, t96178: F, t96231: F, t96274: F, t96377: F, t96420: F, t96466: F, t96508: F, t96554: F, t96594: F) -> (F,) {
    let t96626 = -6.0 * t2322 * t26415 - 6.0 * t4254 * t26415 - 6.0 * t651 * t7474 * t2371 - 9.0 * t25082 * t26405 * t49640 - t118 * (t96083 + t96178) - 18.0 * t28167 * t26405 * t49616 - 3.0 * t7235 * t26392 + t2014 * t532 * (t96231 + t96274 + t96377 + t96420 + t96466 + t96508 + t96554 + t96594) * t1450 - 2.0 * t651 * t2089 * t10259 - 2.0 * t46126 * t2056 - 6.0 * t49851 * t2056 - 6.0 * t10416 * t7367 + 18.0 * t25082 * t28286 * t49560 + 3.0 * t25188 * t7537 + 3.0 * t7484 * t4151 + t2093 * t10192 - 6.0 * t26399 * t2372 - 12.0 * t13435 * t7374 - 6.0 * t18163 * t7374 - 6.0 * t26676 * t1310;
    (t96626,)
}
