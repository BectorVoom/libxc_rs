//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3812/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3812(t48152: f64, t48154: f64, t3860: f64, t6801: f64, t3863: f64, t48158: f64, t46960: f64, t46964: f64, t46967: f64, t123: f64, t2630: f64, t6800: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t73327 = 24.0_f64 * t48152;
    let t73328 = 8.0_f64 * t48154;
    let t73329 = t3860 * t6801;
    let t73330 = 12.0_f64 * t73329;
    let t73331 = t3863 * t6801;
    let t73332 = 32.0_f64 * t73331;
    let t73333 = 48.0_f64 * t48158;
    let t73334 = 12.0_f64 * t46960;
    let t73338 = 32.0_f64 * t46964;
    let t73339 = 20.0_f64 * t46967;
    let t73341 = t6800 * t123 * t2630;
    (t73327, t73328, t73330, t73332, t73333, t73334, t73338, t73339, t73341)
}
