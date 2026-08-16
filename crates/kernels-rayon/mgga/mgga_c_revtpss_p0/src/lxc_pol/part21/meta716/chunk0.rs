//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2552/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2552(t2735: f64, t9792: f64, t1413: f64, t46826: f64, t1376: f64, t40769: f64, t3989: f64, t9986: f64, t10001: f64, t221: f64, t4019: f64, t9912: f64) -> (f64, f64, f64, f64, f64) {
    let t46835 = t2735 * t9792;
    let t46837 = t46835 * t1413 * t46826;
    let t46840 = 0.70398079132139197745e-2_f64 * t40769 * t1376;
    let t46846 = t3989 * t9986;
    let t46853 = t10001 * t4019 * t221 * t9912;
    (t46835, t46837, t46840, t46846, t46853)
}
