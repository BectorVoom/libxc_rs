//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 1007/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk1007(t1: f64, t49821: f64, t544: f64, t1424: f64, t42400: f64, t42420: f64, t42428: f64, t46767: f64, t46773: f64, t46775: f64, t46778: f64, t46785: f64, t46788: f64, t46792: f64, t46793: f64, t46799: f64, t46806: f64, t46811: f64, t46815: f64, t46819: f64, t46821: f64, t48194: f64, t48208: f64) -> f64 {
    let t50773 = t544 * t49821 * t1;
    let t50776 = t46767 + t46773 - t46775 - t46778 - 0.10224780254378866581e1_f64 * t48194 + t46785 - t46788 - 0.63904876589867916127e-1_f64 * t42400 - t46792 - t46793 - 0.12780975317973583225e0_f64 * t42420 + 0.31952438294933958063e0_f64 * t42428 - t46799 - 0.59584149919750711116e-1_f64 * t48208 - t46806 - 0.12780975317973583225e1_f64 * t46811 + 0.85206502119823888169e-1_f64 * t46815 + t46819 - 0.76685851907841499352e0_f64 * t46821 - 0.39722766613167140743e-1_f64 * t50773 * t1424;
    t50776
}
