//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2883/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2883(t10039: f64, t2439: f64, t2777: f64, t1429: f64, t39501: f64, t4056: f64, t9994: f64, t10014: f64, t10136: f64, t215: f64, t3923: f64, t268: f64, t4101: f64, t543: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46401 = t2439 * t2777 * t10039;
    let t46412 = 0.56911289235245161963e-1_f64 * t39501 * t1429;
    let t46416 = t9994 * t4056;
    let t46443 = t10014 * t10136;
    let t46445 = t215 * t3923;
    let t46448 = t4101 * t268 * t46445 * t543;
    (t46401, t46412, t46416, t46443, t46445, t46448)
}
