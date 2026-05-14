//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1084/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1084<F: Float>(t3046: F, t7143: F, t25515: F, t4890: F, t3299: F, t3317: F, t1967: F, t816: F, t1014: F, t65: F, t3252: F, t3204: F, t7131: F, t1078: F, t11239: F, t1035: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t27415 = t3046 * t7143;
    let t27492 = t25515 * t4890;
    let t27493 = t3299 * t27492;
    let t27498 = t3317 * t27492;
    let t27526 = t1967 * t816;
    let t27527 = t65 * t1014;
    let t27531 = t65 * t3252;
    let t27536 = t3204 * t7131;
    let t27638 = t11239 * t1078;
    let t27639 = t27638 * t1035;
    (t27415, t27492, t27493, t27498, t27526, t27527, t27531, t27536, t27638, t27639)
}
