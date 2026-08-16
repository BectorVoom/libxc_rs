//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2101/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2101<F: Float>(t46437: F, t1472: F, t9862: F, t32: F, t4094: F, t10109: F, t1527: F, t1496: F, t41083: F, t4257: F, t9601: F, t4261: F) -> (F, F, F, F, F, F, F) {
    let t46438 = F::cast_from(3.0_f64) * t46437;
    let t46439 = t1472 * t9862;
    let t46447 = t32 * t4094;
    let t46488 = t10109 * t1527;
    let t46546 = t41083 * t1496;
    let t46549 = t9601 * t4257;
    let t46550 = F::cast_from(595.0_f64) / F::cast_from(1152.0_f64) * t46549;
    let t46573 = t9601 * t4261;
    (t46438, t46439, t46447, t46488, t46546, t46550, t46573)
}
