//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1069/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1069(t1161: f64, t353: f64, t35541: f64, t8599: f64, t3886: f64, t4386: f64, t8713: f64, t13121: f64, t22493: f64, t13684: f64, t4414: f64, t13619: f64, t840: f64) -> (f64, f64, f64, f64, f64) {
    let t46862 = t8599 * t353 * t35541 * t1161;
    let t46867 = t4386 * t353 * t8713 * t3886;
    let t46870 = t22493 * t13121;
    let t46872 = t4414 * t13684;
    let t46892 = t840 * t13619;
    (t46862, t46867, t46870, t46872, t46892)
}
