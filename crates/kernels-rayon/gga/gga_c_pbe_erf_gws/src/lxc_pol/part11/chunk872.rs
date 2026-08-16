//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 872/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk872(t13314: f64, t13339: f64, t13355: f64, t13361: f64, t13367: f64, t13373: f64, t13377: f64, t13384: f64, t13391: f64, t13407: f64, t13416: f64, t13439: f64, t13444: f64, t13448: f64) -> f64 {
    let t13672 = t13314 - t13339 + t13355 - t13361 - t13367 + t13373 + t13377 - t13384 - t13391 - t13407 + t13416 - t13439 + t13444 - t13448;
    t13672
}
