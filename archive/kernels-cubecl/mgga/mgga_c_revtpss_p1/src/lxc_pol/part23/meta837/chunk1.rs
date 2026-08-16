//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2709/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2709<F: Float>(t21102: F, t3704: F, t21094: F, t3172: F, t5384: F, t17361: F, t5274: F, t5261: F, t5390: F, t12915: F, t20703: F, t247: F) -> (F, F, F, F, F) {
    let t69674 = t21102 * t3704;
    let t69698 = t5384 * t3172 * t21094;
    let t69700 = t5274 * t17361;
    let t69710 = t5261 * t5390;
    let t69719 = t5384 * t247 * t12915 * t20703;
    (t69674, t69698, t69700, t69710, t69719)
}
