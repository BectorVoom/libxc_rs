//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1067/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1067(t46705: f64, t6148: f64, t830: f64, t13212: f64, t8662: f64, t12198: f64, t3047: f64, t13105: f64, t35014: f64, t13656: f64, t6832: f64, t13641: f64, t2246: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46707 = t6148 * t830 * t46705;
    let t46710 = t8662 * t13212;
    let t46712 = t12198 * t3047;
    let t46714 = t35014 * t13105;
    let t46717 = t6832 * t13656;
    let t46723 = t2246 * t13641;
    (t46707, t46710, t46712, t46714, t46717, t46723)
}
