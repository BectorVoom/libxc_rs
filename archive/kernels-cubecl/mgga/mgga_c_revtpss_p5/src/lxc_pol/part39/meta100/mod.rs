//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta100 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk547;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk548;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk549;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta100<F: Float>(t45: F, t57: F, t2371: F, t508: F, t200: F, t2251: F, t2258: F, t78: F, t202: F, t81: F, t162: F, t187: F, t205: F, t262: F, zeta_threshold: F, t775: F, t705: F, t716: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t2372, t2375, t2382, t2389, t2390, t2392, t2393) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk547::<F>(t45, t57, t2371, t508, t200, t2251, t2258, t78, t202, t81, t162, t187, t205, t262, zeta_threshold);
        let t2394 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk548::<F>(t775);
        let t2398 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk549::<F>(t705, t716);
    (t2372, t2375, t2382, t2389, t2390, t2392, t2393, t2394, t2398)
}
