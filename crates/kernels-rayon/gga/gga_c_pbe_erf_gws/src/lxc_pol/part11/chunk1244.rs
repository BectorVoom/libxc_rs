//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1244/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1244(t11459: f64, t13408: f64, t2168: f64, t6523: f64, t45444: f64, t1105: f64, t13291: f64, t2147: f64, t337: f64, t9119: f64, t3824: f64, t816: f64) -> (f64, f64, f64, f64) {
    let t49717 = 3.0_f64 / 8.0_f64 * t2168 * t6523 * t11459 * t13408;
    let t49722 = 7.0_f64 / 24.0_f64 * t45444;
    let t49729 = t9119 * t2147 * t337 * t13291 * t1105 / 6.0_f64;
    let t49730 = t816 * t3824;
    (t49717, t49722, t49729, t49730)
}
