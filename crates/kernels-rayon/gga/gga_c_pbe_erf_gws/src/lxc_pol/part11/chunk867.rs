//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 867/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk867(t13280: f64, t13345: f64, t13392: f64, t13453: f64, t13484: f64, t13515: f64, t13568: f64, t13603: f64, t898: f64, t338: f64, t353: f64, t1161: f64, t3721: f64) -> (f64, f64, f64, f64) {
    let t13606 = t13280 + t13345 + t13392 + t13453 + t13484 + t13515 + t13568 + t13603;
    let t13607 = t898 * t13606;
    let t13609 = t338 * t353 * t13607;
    let t13612 = t3721 * t1161;
    (t13606, t13607, t13609, t13612)
}
