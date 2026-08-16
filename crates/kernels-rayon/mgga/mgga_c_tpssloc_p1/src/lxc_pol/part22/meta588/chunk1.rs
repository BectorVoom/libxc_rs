//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2101/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2101(t46437: f64, t1472: f64, t9862: f64, t32: f64, t4094: f64, t10109: f64, t1527: f64, t1496: f64, t41083: f64, t4257: f64, t9601: f64, t4261: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46438 = 3.0_f64 * t46437;
    let t46439 = t1472 * t9862;
    let t46447 = t32 * t4094;
    let t46488 = t10109 * t1527;
    let t46546 = t41083 * t1496;
    let t46549 = t9601 * t4257;
    let t46550 = 595.0_f64 / 1152.0_f64 * t46549;
    let t46573 = t9601 * t4261;
    (t46438, t46439, t46447, t46488, t46546, t46550, t46573)
}
