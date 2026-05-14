//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 861/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk861<F: Float>(t46369: F, t793: F, t2347: F, t5267: F, t25820: F, t5888: F, t27101: F, t1635: F, t27041: F, t5898: F, t2350: F, t25854: F, t45419: F, t7785: F, t262: F, t46258: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t46370 = t793 * t46369;
    let t46385 = t2347 * t5267;
    let t46386 = t25820 * t46385;
    let t46388 = t2347 * t5888;
    let t46389 = t27101 * t46388;
    let t46391 = t2347 * t1635;
    let t46392 = t27041 * t46391;
    let t46394 = t2347 * t5898;
    let t46395 = t25820 * t46394;
    let t46397 = t2350 * t5888;
    let t46398 = t25854 * t46397;
    let t46409 = t7785 * t45419;
    let t46412 = t262 * t46258;
    (t46370, t46385, t46386, t46388, t46389, t46391, t46392, t46394, t46395, t46397, t46398, t46409, t46412)
}
