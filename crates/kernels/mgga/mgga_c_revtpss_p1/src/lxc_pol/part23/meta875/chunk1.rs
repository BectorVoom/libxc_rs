//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2779/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2779<F: Float>(t22061: F, t9793: F, t9794: F, t22093: F, t9962: F, t13845: F, t73731: F, t9818: F, t9835: F, t13847: F, t13848: F, t21990: F) -> (F, F, F, F) {
    let t74641 = t9793 * t9794 * t22061;
    let t74656 = t9962 * t22093;
    let t74660 = t13845 * t9818 * t73731 * t9835;
    let t74664 = t13845 * t13847 * t13848 * t21990;
    (t74641, t74656, t74660, t74664)
}
