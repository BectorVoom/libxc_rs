//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2220/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2220(t104409: f64, t104427: f64, t13429: f64, t1518: f64, t18153: f64, t2127: f64, t2163: f64, t2371: f64, t27056: f64, t29456: f64, t4254: f64, t569: f64, t651: f64, t8233: f64, t97661: f64, t97663: f64, t97666: f64, t98421: f64, t98426: f64, t98428: f64, t98430: f64, t98432: f64, t98439: f64, t98440: f64, t98442: f64, t98449: f64, t98452: f64) -> f64 {
    let t104433 = t97661 - 4.0_f64 * t4254 * t29456 - 2.0_f64 * t651 * t27056 * t1518 - t97663 - t97666 + t98421 - 2.0_f64 * t13429 * t2163 - t98426 - t98428 - t98430 - t98432 - t2127 * t18153 + (t104409 + t104427) * t569 - t98439 + t98440 - t98442 - 2.0_f64 * t651 * t8233 * t2371 + t98449 - t98452;
    t104433
}
