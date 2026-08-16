//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1694/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1694(t2289: f64, t2367: f64, t10210: f64, t625: f64, t10214: f64, t10255: f64, t10207: f64, t111: f64, t2340: f64, t2366: f64, t39455: f64, t36227: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46148 = t2289 * t2367;
    let t46150 = t625 * t10210;
    let t46152 = t625 * t10214;
    let t46154 = t625 * t10255;
    let t46157 = 1.0_f64 / t10207 / t111;
    let t46158 = t2340 * t2340;
    let t46166 = t2366 * t2366;
    let t46173 = -12.0_f64 * t39455;
    let t46196 = 1.0_f64 / t36227;
    (t46148, t46150, t46152, t46154, t46157, t46158, t46166, t46173, t46196)
}
