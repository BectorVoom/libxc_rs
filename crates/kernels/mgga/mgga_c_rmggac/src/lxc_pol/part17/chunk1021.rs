//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1021/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1021<F: Float>(t2134: F, t27: F, t5840: F, t649: F, t46412: F, t8630: F, t46416: F, t7192: F, t2333: F, t39953: F, t7487: F, t9720: F) -> (F, F, F, F, F) {
    let t46811 = t2134 * t27 * t649 * t5840;
    let t46815 = t8630 * t46412;
    let t46817 = t7192 * t46416;
    let t46819 = t39953 * t2333;
    let t46821 = t7487 * t9720;
    (t46811, t46815, t46817, t46819, t46821)
}
