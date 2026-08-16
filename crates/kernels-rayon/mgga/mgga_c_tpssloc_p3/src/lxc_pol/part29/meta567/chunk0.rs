//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1984/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1984(t12566: f64, t604: f64, t2239: f64, t3951: f64, t13034: f64, t225: f64, t10109: f64, t1527: f64, t13036: f64, t4119: f64, t828: f64, t1484: f64, t2678: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46099 = t12566 * t604;
    let t46104 = t3951 * t2239;
    let t46452 = t13034 * t225;
    let t46488 = t10109 * t1527;
    let t46508 = t13036 * t225;
    let t46565 = t4119 * t828;
    let t46644 = t1484 * t2678;
    (t46099, t46104, t46452, t46488, t46508, t46565, t46644)
}
