//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2977/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2977<F: Float>(t5760: F, t9292: F, t10069: F, t14207: F, t40921: F, t5737: F, t225: F, t2453: F, t136: F, t137: F, t1398: F, t14140: F, t2438: F, t4003: F) -> (F, F, F, F, F) {
    let t49172 = t9292 * t5760;
    let t49176 = t10069 * t14207;
    let t49178 = t40921 * t5737;
    let t49180 = t2453 * t225;
    let t49186 = t49180 * t14140 * t4003 * t136 * t137 * t2438 * t1398;
    (t49172, t49176, t49178, t49180, t49186)
}
