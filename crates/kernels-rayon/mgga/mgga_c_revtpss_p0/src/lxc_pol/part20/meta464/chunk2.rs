//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1766/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1766(t46654: f64, t46714: f64, t46782: f64, t46848: f64, t46911: f64, t47212: f64, t47279: f64, t47340: f64, t10111: f64, t22: f64, t4092: f64, t39515: f64, t4083: f64) -> (f64, f64, f64) {
    let t47343 = t46654 + t46714 + t46782 + t46848 + t46911 + t47212 + t47279 + t47340;
    let t47348 = t10111 * t4092 * t22;
    let t47351 = 0.11564373972601816912e-1_f64 * t39515 * t4083;
    (t47343, t47348, t47351)
}
