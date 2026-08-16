//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 949/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk949<F: Float>(t46867: F, t475: F, t1064: F, t2268: F, t13749: F, t599: F, t2343: F, t12018: F, t894: F, t13756: F, t419: F, t13751: F, t380: F) -> (F, F, F, F, F, F, F, F) {
    let t46915 = t46867 * t475;
    let t46918 = F::cast_from(0.85365019907028448797e-1_f64) * t2268 * t1064 * t46915;
    let t46919 = t599 * t13749;
    let t46920 = t46919 * t475;
    let t46923 = F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t2343 * t46920;
    let t46928 = t2268 * t894 * t12018;
    let t46931 = F::cast_from(0.28455006635676149599e-1_f64) * t419 * t13756;
    let t46933 = F::cast_from(0.37940008847568199465e-1_f64) * t380 * t13751;
    (t46915, t46918, t46919, t46920, t46923, t46928, t46931, t46933)
}
