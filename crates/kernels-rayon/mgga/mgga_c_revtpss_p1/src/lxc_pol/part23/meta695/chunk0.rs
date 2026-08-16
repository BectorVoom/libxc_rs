//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2442/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2442(t2452: f64, t40633: f64, t46808: f64, t547: f64, t268: f64, t40634: f64, t550: f64, t9718: f64, t247: f64, t548: f64, t9722: f64, t1379: f64, t40846: f64, t816: f64) -> (f64, f64, f64, f64) {
    let t46810 = 0.30119321664969771194e-5_f64 * t40633 * t2452 * t547 * t46808;
    let t46817 = 0.53552153920316253184e-5_f64 * t9718 * t40634 * t550 * t268;
    let t46820 = 0.28974367305964659283e0_f64 * t548 * t9722 * t247;
    let t46824 = 0.12516778469694349359e-1_f64 * t1379 * t40846 * t550 * t816;
    (t46810, t46817, t46820, t46824)
}
