//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta480 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1758;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1759;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1760;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta480<F: Float>(t25207: F, t27375: F, t11064: F, t30: F, t1583: F, t890: F, t605: F, t4537: F, t1468: F, t775: F, t33: F, t892: F, t4433: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t27376, t27383) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1758::<F>(t25207, t27375, t11064, t30);
        let t27384 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1759::<F>(t1583, t890);
        let (t27385, t27387, t27391, t27395, t27402, t27763, t27764) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1760::<F>(t27383, t27384, t1583, t605, t30, t4537, t1468, t775, t890, t33, t892, t4433);
    (t27376, t27383, t27384, t27385, t27387, t27391, t27395, t27402, t27763, t27764)
}
