//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 980/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk980<F: Float>(t27101: F, t46388: F, t1635: F, t2347: F, t27041: F, t5898: F, t25820: F, t2350: F, t5888: F, t25854: F, t45419: F, t7785: F) -> (F, F, F, F, F, F, F, F) {
    let t46389 = t27101 * t46388;
    let t46391 = t2347 * t1635;
    let t46392 = t27041 * t46391;
    let t46394 = t2347 * t5898;
    let t46395 = t25820 * t46394;
    let t46397 = t2350 * t5888;
    let t46398 = t25854 * t46397;
    let t46409 = t7785 * t45419;
    (t46389, t46391, t46392, t46394, t46395, t46397, t46398, t46409)
}
