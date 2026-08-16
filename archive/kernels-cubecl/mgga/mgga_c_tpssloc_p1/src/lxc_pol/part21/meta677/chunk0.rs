//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2483/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2483<F: Float>(t4199: F, t9494: F, t13471: F, t870: F, t12945: F, t2427: F, t12858: F, t2528: F, t2371: F, t4205: F, t9909: F, t13123: F, t9885: F) -> (F, F, F, F, F, F, F) {
    let t46208 = t4199 * t9494;
    let t46213 = t13471 * t870;
    let t46217 = t2427 * t12945;
    let t46234 = t12858 * t2528;
    let t46236 = t12858 * t2371;
    let t46244 = t4205 * t9909;
    let t46278 = t13123 * t9885;
    (t46208, t46213, t46217, t46234, t46236, t46244, t46278)
}
