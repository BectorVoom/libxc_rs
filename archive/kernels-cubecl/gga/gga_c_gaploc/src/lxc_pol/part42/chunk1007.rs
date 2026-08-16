//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 1007/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk1007<F: Float>(t1: F, t49821: F, t544: F, t1424: F, t42400: F, t42420: F, t42428: F, t46767: F, t46773: F, t46775: F, t46778: F, t46785: F, t46788: F, t46792: F, t46793: F, t46799: F, t46806: F, t46811: F, t46815: F, t46819: F, t46821: F, t48194: F, t48208: F) -> F {
    let t50773 = t544 * t49821 * t1;
    let t50776 = t46767 + t46773 - t46775 - t46778 - F::cast_from(0.10224780254378866581e1_f64) * t48194 + t46785 - t46788 - F::cast_from(0.63904876589867916127e-1_f64) * t42400 - t46792 - t46793 - F::cast_from(0.12780975317973583225e0_f64) * t42420 + F::cast_from(0.31952438294933958063e0_f64) * t42428 - t46799 - F::cast_from(0.59584149919750711116e-1_f64) * t48208 - t46806 - F::cast_from(0.12780975317973583225e1_f64) * t46811 + F::cast_from(0.85206502119823888169e-1_f64) * t46815 + t46819 - F::cast_from(0.76685851907841499352e0_f64) * t46821 - F::cast_from(0.39722766613167140743e-1_f64) * t50773 * t1424;
    t50776
}
