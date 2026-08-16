//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1788/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1788(t1448: f64, t1450: f64, t198: f64, t39483: f64, t39520: f64, t39528: f64, t39531: f64, t46961: f64, t46963: f64, t46965: f64, t46968: f64, t46970: f64, t46972: f64, t46974: f64, t46976: f64, t46978: f64, t47468: f64, t47518: f64, t47566: f64, t47622: f64, t532: f64, t9400: f64) -> f64 {
    let t47632 = t198 * t532 * (t47468 + t47518 + t47566 + t47622) * t1450 + t46961 - t46963 - t46965 + t46968 + t46970 + t46972 - t39483 + t39520 + 24.0_f64 * t198 * t9400 * t1448 * t1450 - t39528 - t46974 + t39531 + t46976 - t46978;
    t47632
}
