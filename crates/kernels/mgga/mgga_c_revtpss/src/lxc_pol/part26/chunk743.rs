//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 743/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk743<F: Float>(t10380: F, t38: F, t2851: F, t78: F, t2299: F, t606: F, t3361: F, t81: F, t2306: F, t10326: F, t10356: F, t2258: F, t633: F, t637: F, t77: F, t10317: F, t10318: F, t10321: F, t10328: F, t10331: F, t10336: F, t2252: F, t2260: F, t2263: F, t2292: F, t2312: F, t608: F, t628: F, t641: F, t71: F, t85: F) -> (F, F) {
    let t10381 = t38 * t10380;
    let t10389 = 1.0 / t78 / t2851;
    let t10392 = t2299 * t606;
    let t10398 = 1.0 / t81 / t3361;
    let t10401 = t2306 * t606;
    let t10406 = -280.0 / 27.0 * t10389 * t10356 + 28.0 / 3.0 * t10392 * t2258 - 4.0 / 3.0 * t633 * t10326 + 280.0 / 27.0 * t10398 * t10356 + 28.0 / 3.0 * t10401 * t2258 + 4.0 / 3.0 * t637 * t10326;
    let t10407 = t77 * t10406;
    let t10410 = -t10317 * t10318 / 4.0 - t10321 * t85 / 4.0 - t2252 * t641 / 4.0 - t10328 * t85 / 12.0 - t10331 * t85 / 4.0 - t2260 * t641 / 4.0 - t10336 * t85 / 4.0 - t2263 * t641 / 2.0 - t608 * t2312 / 4.0 + t10381 * t85 / 24.0 + t2292 * t641 / 8.0 + t628 * t2312 / 8.0 + t71 * t10407 / 24.0;
    (t10406, t10410)
}
