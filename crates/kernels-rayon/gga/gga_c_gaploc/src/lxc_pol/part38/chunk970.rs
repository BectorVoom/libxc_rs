//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 970/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk970(t2482: f64, t3541: f64, t9267: f64, t11549: f64, t9272: f64, t11402: f64, t2441: f64, t13397: f64, t21373: f64, t6914: f64, t11218: f64, t123: f64, t883: f64) -> (f64, f64, f64, f64, f64) {
    let t46387 = t9267 * t3541 * t2482;
    let t46390 = t9272 * t11549 * t2482;
    let t46396 = 0.35750489951850426669e0_f64 * t2441 * t11402;
    let t46398 = t6914 * t21373 * t13397;
    let t46401 = t11218 * t123 * t883;
    (t46387, t46390, t46396, t46398, t46401)
}
