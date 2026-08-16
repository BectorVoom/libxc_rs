//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta95 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk532;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk533;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk534;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta95<F: Float>(t2246: F, t29: F, t644: F, t606: F, t70: F, t2: F, t580: F, t17: F, t30: F, t33: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
        let (t2247, t2248, t2251) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk532::<F>(t2246, t29, t644, t606);
        let (t2252, t2255, t2256, t2257) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk533::<F>(t2251, t70, t2, t580, t17);
        let t2258 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk534::<F>(t30, t33, t2257, zeta_threshold);
    (t2247, t2248, t2251, t2252, t2255, t2256, t2257, t2258)
}
