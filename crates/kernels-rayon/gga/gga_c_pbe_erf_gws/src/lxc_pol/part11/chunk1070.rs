//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1070/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1070(t12111: f64, t3083: f64, t2501: f64, t3717: f64, t2370: f64, t830: f64, t3052: f64, t9955: f64, t45235: f64, t6801: f64, t12213: f64, t3721: f64) -> (f64, f64, f64, f64, f64) {
    let t46914 = t3083 * t12111;
    let t46923 = t2501 * t3717;
    let t46925 = t2370 * t830 * t46923;
    let t46928 = t9955 * t3052;
    let t46930 = t45235 * t6801;
    let t46974 = t12213 * t3721;
    (t46914, t46925, t46928, t46930, t46974)
}
