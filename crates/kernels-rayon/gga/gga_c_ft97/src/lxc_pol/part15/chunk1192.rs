//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1192/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1192(t13605: f64, t1526: f64, t20489: f64, t21181: f64, t21931: f64, t21958: f64, t21969: f64, t21973: f64, t22168: f64, t2320: f64, t2639: f64, t3806: f64, t44700: f64, t44716: f64, t72992: f64, t82488: f64, t82491: f64) -> f64 {
    let t90537 = -t1526 * t2320 * t21969 / 4.0_f64 - t1526 * t2320 * t2639 * t20489 / 12.0_f64 + t22168 - t1526 * t3806 * t21958 / 3.0_f64 - 7.0_f64 / 27.0_f64 * t1526 * t13605 * t44700 * t21181 - t1526 * t2320 * t21973 / 4.0_f64 - t82488 / 9.0_f64 - t82491 / 6.0_f64 + t21931 - t44716 + t72992 / 18.0_f64;
    t90537
}
