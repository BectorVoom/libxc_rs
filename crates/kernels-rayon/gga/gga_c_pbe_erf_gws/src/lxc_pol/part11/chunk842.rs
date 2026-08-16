//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 842/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk842(t13304: f64, t2168: f64, t11808: f64, t3128: f64, t1149: f64, t11700: f64, t11592: f64, t3793: f64, t11493: f64, t13220: f64, t339: f64, t1130: f64, t3717: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13306 = t2168 * t13304 / 32.0_f64;
    let t13308 = t3128 * t11808 / 16.0_f64;
    let t13309 = t11700 * t1149;
    let t13313 = t11592 * t3793 / 48.0_f64;
    let t13314 = 7.0_f64 / 48.0_f64 * t11493;
    let t13325 = t339 * t13220;
    let t13328 = t1130 * t3717;
    (t13306, t13308, t13309, t13313, t13314, t13325, t13328)
}
