//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2530/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2530(t1419: f64, t4056: f64, t1429: f64, t39501: f64, t1398: f64, t9840: f64, t2482: f64, t4114: f64, t686: f64, t72: f64, t543: f64, t1437: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46407 = t1419 * t4056;
    let t46412 = 0.56911289235245161963e-1_f64 * t39501 * t1429;
    let t46422 = t9840 * t1398;
    let t46424 = t2482 * t4114 * t72 * t686 * t46422;
    let t46432 = t4056 * t1398;
    let t46433 = t46432 * t543;
    let t46435 = t2482 * t1437 * t72 * t686 * t46433;
    (t46407, t46412, t46422, t46424, t46432, t46433, t46435)
}
