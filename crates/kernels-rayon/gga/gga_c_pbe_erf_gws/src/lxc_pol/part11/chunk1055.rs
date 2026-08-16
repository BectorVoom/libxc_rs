//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1055/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1055(t13450: f64, t6416: f64, t13531: f64, t6627: f64, t13304: f64, t6484: f64, t13561: f64, t13418: f64, t9630: f64, t13220: f64, t5: f64, t20560: f64, t337: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45421 = t6416 * t13450;
    let t45438 = t6627 * t13531;
    let t45444 = t6484 * t13304;
    let t45450 = t6627 * t13561;
    let t45452 = t9630 * t13418;
    let t45485 = t5 * t13220;
    let t45487 = t20560 * t337 * t45485;
    (t45421, t45438, t45444, t45450, t45452, t45487)
}
