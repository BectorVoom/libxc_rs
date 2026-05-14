//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 874/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk874<F: Float>(t25854: F, t46397: F, t5144: F, t8975: F, t5267: F, t5888: F, t45419: F, t7785: F, t118: F, t27048: F, t27101: F, t326: F, t46375: F, t46379: F, t46382: F, t46386: F, t46389: F, t46392: F, t46395: F) -> (F, F, F, F) {
    let t46398 = t25854 * t46397;
    let t46400 = t8975 * t5144;
    let t46403 = t8975 * t5267;
    let t46406 = t8975 * t5888;
    let t46409 = t7785 * t45419;
    let t46411 = -0.11974241701863808564e0 * t326 * t46375 - 0.39914139006212695214e-1 * t118 * t46379 - 0.59871208509319042821e-1 * t326 * t46382 + 0.17961362552795712846e0 * t46386 + 0.11974241701863808564e0 * t46389 + 0.71845450211182851384e0 * t46392 + 0.17961362552795712846e0 * t46395 - 0.17961362552795712846e0 * t46398 - 0.47896966807455234256e0 * t27101 * t46400 + 0.71845450211182851384e0 * t25854 * t46403 + 0.71845450211182851384e0 * t27048 * t46406 + 0.40911992481368012592e-1 * t46409;
    (t46400, t46403, t46406, t46411)
}
