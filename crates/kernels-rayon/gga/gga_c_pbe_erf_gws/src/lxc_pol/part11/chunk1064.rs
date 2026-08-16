//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1064/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1064(t13337: f64, t2164: f64, t13171: f64, t2306: f64, t339: f64, t13490: f64, t3116: f64, t6183: f64, t11794: f64, t8824: f64, t13463: f64, t13405: f64, t8967: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46536 = t2164 * t13337;
    let t46544 = t2306 * t13171 * t339;
    let t46549 = t3116 * t6183 * t13490;
    let t46566 = t11794 * t8824;
    let t46596 = t2164 * t13463;
    let t46598 = t8967 * t13405;
    (t46536, t46544, t46549, t46566, t46596, t46598)
}
