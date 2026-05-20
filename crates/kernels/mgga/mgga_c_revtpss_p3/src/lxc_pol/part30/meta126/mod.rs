//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta126 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk711;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk712;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta126<F: Float>(t251: F, t836: F, t231: F, t2783: F, t2782: F, t233: F, t860: F, t869: F, t689: F, t136: F, t2457: F, t2710: F, t786: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t2786, t2787, t2789, t2790, t2791, t2793, t2796) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk711::<F>(t251, t836, t231, t2783, t2782, t233, t860, t869, t689, t136, t2457, t2710);
        let (t2797, t2798) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk712::<F>(t251, t2783, t786);
    (t2786, t2787, t2789, t2790, t2791, t2793, t2796, t2797, t2798)
}
