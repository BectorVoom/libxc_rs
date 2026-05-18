//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1076/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1076<F: Float>(t13749: F, t599: F, t475: F, t2268: F, t2343: F, t42604: F, t42605: F, t42606: F, t42607: F, t42610: F, t42613: F, t46908: F, t46912: F, t46913: F, t46918: F) -> (F, F, F) {
    let t46919 = t599 * t13749;
    let t46920 = t46919 * t475;
    let t46923 = F::new(0.56910013271352299198e-1) * t2268 * t2343 * t46920;
    let t46924 = t42604 + t42605 - t42606 + F::new(0.28455006635676149599e-1) * t42607 + F::new(0.28455006635676149599e-1) * t42610 + F::new(0.28455006635676149599e-1) * t42613 - F::new(0.56910013271352299198e-1) * t46908 + t46912 + F::new(0.37940008847568199465e-1) * t46913 - t46918 + t46923;
    (t46919, t46920, t46924)
}
