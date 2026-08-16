//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 980/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk980(t2109: f64, t8282: f64, t2098: f64, t1775: f64, t9233: f64, t24: f64, t38534: f64, t38550: f64, t38566: f64, t40262: f64, t40267: f64, t40368: f64, t40370: f64, t40375: f64, t40377: f64, t40379: f64, t40384: f64, t40392: f64, t462: f64, t582: f64, t586: f64, t92: f64, t9224: f64) -> f64 {
    let t40397 = t8282 * t2109;
    let t40399 = t8282 * t2098;
    let t40401 = t1775 * t9233;
    let t40403 = -8.0_f64 / 3.0_f64 * t40368 + 112.0_f64 / 27.0_f64 * t40370 - t92 * t24 * t586 * t40262 + 4.0_f64 / 3.0_f64 * t40375 + 16.0_f64 / 3.0_f64 * t40377 + 24.0_f64 * t92 * t24 * t40379 * t40267 + 8.0_f64 * t40384 + 2.0_f64 * t462 * t582 * t38566 - t462 * t582 * t38534 / 3.0_f64 - 16.0_f64 / 9.0_f64 * t40392 + 40.0_f64 / 9.0_f64 * t462 * t9224 * t38550 - 8.0_f64 / 9.0_f64 * t40397 - 16.0_f64 / 27.0_f64 * t40399 + 4.0_f64 / 9.0_f64 * t40401;
    t40403
}
