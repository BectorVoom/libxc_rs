//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 949/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk949(t46867: f64, t475: f64, t1064: f64, t2268: f64, t13749: f64, t599: f64, t2343: f64, t12018: f64, t894: f64, t13756: f64, t419: f64, t13751: f64, t380: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46915 = t46867 * t475;
    let t46918 = 0.85365019907028448797e-1_f64 * t2268 * t1064 * t46915;
    let t46919 = t599 * t13749;
    let t46920 = t46919 * t475;
    let t46923 = 0.56910013271352299198e-1_f64 * t2268 * t2343 * t46920;
    let t46928 = t2268 * t894 * t12018;
    let t46931 = 0.28455006635676149599e-1_f64 * t419 * t13756;
    let t46933 = 0.37940008847568199465e-1_f64 * t380 * t13751;
    (t46915, t46918, t46919, t46920, t46923, t46928, t46931, t46933)
}
