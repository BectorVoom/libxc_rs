//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1207/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1207(t90: f64, t29: f64, t10414: f64, t116: f64, t10179: f64, t4147: f64, t560: f64, t9655: f64, t1398: f64, t9840: f64, t4056: f64, t543: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45970 = t90 * t90;
    let t45972 = t29 / t45970;
    let t46126 = t10414 * t116;
    let t46304 = t10179 * t4147;
    let t46361 = 1.0_f64 / t9655 / t560;
    let t46422 = t9840 * t1398;
    let t46432 = t4056 * t1398;
    let t46433 = t46432 * t543;
    (t45972, t46126, t46304, t46361, t46422, t46433)
}
