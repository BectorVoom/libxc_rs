//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta574 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1922;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta574<F: Float>(t14874: F, t25270: F, t14746: F, t7025: F, t14769: F, t7045: F, t14727: F, t25227: F, t2661: F, t4430: F, t93034: F, t14861: F) -> (F, F, F, F, F, F) {
        let (t98993, t98995, t98997, t99000, t99002, t99006) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1922::<F>(t14874, t25270, t14746, t7025, t14769, t7045, t14727, t25227, t2661, t4430, t93034, t14861);
    (t98993, t98995, t98997, t99000, t99002, t99006)
}
