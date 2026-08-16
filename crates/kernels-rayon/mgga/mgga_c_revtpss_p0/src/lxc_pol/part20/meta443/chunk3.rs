//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1697/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1697(t114: f64, t46232: f64, t10259: f64, t10260: f64, t10263: f64, t10416: f64, t10426: f64, t118: f64, t1310: f64, t1312: f64, t13207: f64, t13216: f64, t13435: f64, t1453: f64, t2322: f64, t2331: f64, t2371: f64, t3813: f64, t4254: f64, t43735: f64, t45923: f64, t46125: f64, t46126: f64, t46129: f64, t46137: f64, t508: f64, t5523: f64, t569: f64, t651: f64, t670: f64, t93: f64) -> (f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t46233 = piecewise3(t115, 0.0_f64, t46232);
    let t46250 = -8.0_f64 * t651 * t13207 * t670 - 12.0_f64 * t651 * t3813 * t2371 + 4.0_f64 * t10426 * t1453 - 24.0_f64 * t10416 * t2331 - 24.0_f64 * t2322 * t10263 - t118 * (t43735 + t45923) + (8.0_f64 * t10259 * t2322 + 8.0_f64 * t10259 * t5523 + 12.0_f64 * t10416 * t2371 + 2.0_f64 * t1312 * t46233 + 24.0_f64 * t13435 * t2371 + 8.0_f64 * t46126 * t670 + 6.0_f64 * t46137 * t93 + t46125 + 12.0_f64 * t46129) * t569 - t46125 * t508 - 24.0_f64 * t2322 * t13216 - 8.0_f64 * t2322 * t10260 - 8.0_f64 * t4254 * t10260 - 8.0_f64 * t651 * t1310 * t10259 - 24.0_f64 * t4254 * t13216;
    (t46233, t46250)
}
