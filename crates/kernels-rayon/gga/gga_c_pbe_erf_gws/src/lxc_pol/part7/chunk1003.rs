//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1003/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1003(t1903: f64, t1907: f64, t1914: f64, t5441: f64, t256: f64, t5444: f64, t719: f64, t16580: f64, t17467: f64, t17469: f64, t17473: f64, t17476: f64, t17481: f64, t17484: f64, t17488: f64, t247: f64, t251: f64) -> f64 {
    let t18284 = t1907 * t1903;
    let t18286 = t1914 * t5441;
    let t18293 = t5444 * t719 * t256;
    let t18295 = t17467 - t17469 + t17473 - t17476 - t17481 + t17484 - t17488 - 4.0_f64 / 9.0_f64 * t18284 - 0.5402469135802469136e-1_f64 * t18286 + t16580 * t247 * t251 * t256 / 3.0_f64 + 4.0_f64 / 3.0_f64 * t18293;
    t18295
}
