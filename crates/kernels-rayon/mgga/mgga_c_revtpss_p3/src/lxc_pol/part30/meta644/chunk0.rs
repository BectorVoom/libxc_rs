//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2261/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2261(t104416: f64, t1519: f64, t1911: f64, t2372: f64, t27060: f64, t27066: f64, t29427: f64, t4257: f64, t96706: f64, t98559: f64, t98562: f64, t98567: f64, t98569: f64, t98571: f64, t98574: f64, t98578: f64, t98581: f64, t98584: f64, t98590: f64, t98594: f64, t98597: f64, t98599: f64, t98601: f64) -> f64 {
    let t105734 = -4.0_f64 * t104416 * t1519 - 2.0_f64 * t1519 * t96706 + t1911 * t27066 - 2.0_f64 * t2372 * t29427 - 4.0_f64 * t27060 * t4257 - t98559 + t98562 + t98567 - t98569 - t98571 - t98574 + t98578 + t98581 - t98584 + t98590 + t98594 - t98597 - t98599 - t98601;
    t105734
}
