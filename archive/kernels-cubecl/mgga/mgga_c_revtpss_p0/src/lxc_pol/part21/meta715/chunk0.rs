//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2550/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2550<F: Float>(t1389: F, t268: F, t2452: F, t40633: F, t547: F, t9793: F, t9794: F, t9930: F, t40634: F, t550: F, t9718: F, t247: F, t548: F, t9722: F) -> (F, F, F, F) {
    let t46808 = t1389 * t268;
    let t46810 = F::cast_from(0.30119321664969771194e-5_f64) * t40633 * t2452 * t547 * t46808;
    let t46812 = t9793 * t9794 * t9930;
    let t46817 = F::cast_from(0.53552153920316253184e-5_f64) * t9718 * t40634 * t550 * t268;
    let t46820 = F::cast_from(0.28974367305964659283e0_f64) * t548 * t9722 * t247;
    (t46810, t46812, t46817, t46820)
}
