//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta116 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk754;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk755;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk756;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta116<F: Float>(t2782: F, t2786: F, t233: F, t860: F, t869: F, t689: F, t136: F, t251: F, t2457: F, t2710: F, t2783: F, t786: F, t231: F, t268: F, t675: F, t836: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t2787, t2789, t2790, t2791, t2793, t2796, t2797) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk754::<F>(t2782, t2786, t233, t860, t869, t689, t136, t251, t2457, t2710, t2783);
        let t2798 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk755::<F>(t2797, t786);
        let t2801 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk756::<F>(t231, t268, t675, t836);
    (t2787, t2789, t2790, t2791, t2793, t2796, t2797, t2798, t2801)
}
