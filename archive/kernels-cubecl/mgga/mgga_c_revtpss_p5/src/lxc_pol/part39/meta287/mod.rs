//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta287 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1033;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1034;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta287<F: Float>(t20: F, t596: F, t12: F, t583: F, t27: F, t2231: F, t2237: F, t592: F, t2236: F, t3: F, t25: F, t2240: F, t602: F, t2246: F, t599: F, t88: F, t89: F, t90: F, t29: F, t46: F, t47: F, t58: F, t59: F, t10199: F, t2851: F, t78: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10284, t10287, t10288, t10290, t10295, t10298) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1033::<F>(t20, t596, t12, t583, t27, t2231, t2237, t592, t2236, t3, t25, t2240, t602);
        let (t10301, t10309, t10355, t10368, t10379, t10389) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1034::<F>(t2246, t599, t88, t89, t90, t29, t46, t47, t58, t59, t10199, t2851, t78);
    (t10284, t10287, t10288, t10290, t10295, t10298, t10301, t10309, t10355, t10368, t10379, t10389)
}
