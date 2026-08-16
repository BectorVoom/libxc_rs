//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1076/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1076(t13749: f64, t599: f64, t475: f64, t2268: f64, t2343: f64, t42604: f64, t42605: f64, t42606: f64, t42607: f64, t42610: f64, t42613: f64, t46908: f64, t46912: f64, t46913: f64, t46918: f64) -> (f64, f64, f64) {
    let t46919 = t599 * t13749;
    let t46920 = t46919 * t475;
    let t46923 = 0.56910013271352299198e-1_f64 * t2268 * t2343 * t46920;
    let t46924 = t42604 + t42605 - t42606 + 0.28455006635676149599e-1_f64 * t42607 + 0.28455006635676149599e-1_f64 * t42610 + 0.28455006635676149599e-1_f64 * t42613 - 0.56910013271352299198e-1_f64 * t46908 + t46912 + 0.37940008847568199465e-1_f64 * t46913 - t46918 + t46923;
    (t46919, t46920, t46924)
}
