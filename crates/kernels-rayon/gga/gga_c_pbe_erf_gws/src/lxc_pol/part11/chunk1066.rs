//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1066/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1066(t2370: f64, t46654: f64, t830: f64, t1114: f64, t44900: f64, t825: f64, t3083: f64, t9820: f64, t12138: f64, t35277: f64, t3733: f64, t2501: f64, t3703: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46656 = t2370 * t830 * t46654;
    let t46667 = t1114 * t44900 * t825;
    let t46678 = t3083 * t9820;
    let t46685 = t3083 * t12138;
    let t46703 = t35277 * t3733;
    let t46705 = t2501 * t3703;
    (t46656, t46667, t46678, t46685, t46703, t46705)
}
