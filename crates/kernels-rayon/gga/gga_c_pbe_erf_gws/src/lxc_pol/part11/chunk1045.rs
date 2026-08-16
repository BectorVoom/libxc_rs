//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1045/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1045(t2147: f64, t337: f64, t44313: f64, t13481: f64, t2319: f64, t13069: f64, t19: f64, t796: f64, t801: f64, t13156: f64, t817: f64, t13536: f64, t2142: f64) -> (f64, f64, f64, f64, f64) {
    let t44315 = t2147 * t337 * t44313;
    let t44372 = t2319 * t13481;
    let t44395 = t13069 * t796 * t19 * t801;
    let t44405 = t13156 * t817;
    let t44465 = t13536 * t2142;
    (t44315, t44372, t44395, t44405, t44465)
}
