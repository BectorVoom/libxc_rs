//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 666/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk666(t1882: f64, t5327: f64, t5311: f64, t1240: f64, t2766: f64, t5410: f64, t8392: f64, t1212: f64, t2842: f64, t5415: f64, t312: f64, t5225: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19482 = t1882 * t5327;
    let t19484 = t1882 * t5311;
    let t19500 = t2766 * t1240;
    let t19504 = t8392 * t5410;
    let t19506 = t2842 * t1212;
    let t19511 = t8392 * t5415;
    let t19517 = t312 * t5225;
    (t19482, t19484, t19500, t19504, t19506, t19511, t19517)
}
