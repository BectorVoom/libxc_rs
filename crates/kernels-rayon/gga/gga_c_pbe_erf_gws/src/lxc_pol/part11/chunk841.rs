//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 841/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk841(t13290: f64, t5: f64, t337: f64, t2121: f64, t9119: f64, t1149: f64, t12024: f64, t11478: f64, t2170: f64, t3814: f64, t2168: f64, t3131: f64, t3139: f64, t3855: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13291 = t5 * t13290;
    let t13292 = t337 * t13291;
    let t13293 = t2121 * t13292;
    let t13295 = t9119 * t13293 / 48.0_f64;
    let t13296 = t12024 * t1149;
    let t13300 = t2170 * t11478 * t3814;
    let t13302 = t2168 * t13300 / 16.0_f64;
    let t13304 = t3139 * t3131 * t3855;
    (t13291, t13292, t13293, t13295, t13296, t13300, t13302, t13304)
}
