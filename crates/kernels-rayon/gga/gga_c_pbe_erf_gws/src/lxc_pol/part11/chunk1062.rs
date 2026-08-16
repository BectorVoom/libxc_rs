//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1062/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1062(t13363: f64, t6416: f64, t13242: f64, t3116: f64, t6331: f64, t3786: f64, t3912: f64, t6158: f64, t2118: f64, t360: f64, t13549: f64, t21536: f64) -> (f64, f64, f64, f64, f64) {
    let t46280 = t6416 * t13363;
    let t46324 = t3116 * t6331 * t13242;
    let t46327 = t3912 * t6158 * t3786;
    let t46382 = t3912 * t2118 * t3786 * t360;
    let t46399 = t21536 * t13549;
    (t46280, t46324, t46327, t46382, t46399)
}
