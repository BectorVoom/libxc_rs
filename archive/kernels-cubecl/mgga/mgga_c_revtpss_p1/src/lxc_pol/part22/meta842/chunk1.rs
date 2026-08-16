//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2975/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2975<F: Float>(t13775: F, t9793: F, t9794: F, t5690: F, t9741: F, t14016: F, t46691: F, t14020: F, t3957: F, t2659: F, t5744: F, t816: F) -> (F, F, F, F, F) {
    let t49124 = t9793 * t9794 * t13775;
    let t49126 = t9741 * t5690;
    let t49128 = t46691 * t14016;
    let t49134 = t3957 * t14020;
    let t49137 = t816 * t2659 * t5744;
    (t49124, t49126, t49128, t49134, t49137)
}
