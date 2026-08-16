//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 472/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk472(t2331: f64, t2465: f64, t247: f64, t2470: f64, t2527: f64, t2570: f64, t2617: f64, t2619: f64, t263: f64, t719: f64, t771: f64, t342: f64, t630: f64, t784: f64) -> (f64, f64) {
    let t2624 = -t2331 * t263 - t2465 * t263 - t247 * t2617 - 2.0_f64 * t719 * t771 - 4.0_f64 * t2470 - 2.0_f64 * t2527 + 4.0_f64 * t2570 + 2.0_f64 * t2619;
    let t2638 = t342 * t630 * t784 / 12.0_f64;
    (t2624, t2638)
}
