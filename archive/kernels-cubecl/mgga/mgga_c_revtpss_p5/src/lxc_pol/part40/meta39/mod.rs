//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta39 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk237;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk238;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk239;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk240;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk241;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta39<F: Float>(t158: F, t750: F, t162: F, t716: F, t187: F, t192: F, t72: F, t186: F, t675: F, t685: F, t177: F, t738: F, t744: F, t745: F, t206: F, t262: F, t78: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t751, t752, t754, t755, t757) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk237::<F>(t158, t750, t162, t716, t187, t192, t72, t186, t675, t685);
        let (t759, t760) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk238::<F>(t755, t757, t177, t192);
        let t762 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk239::<F>(t738, t744, t745);
        let (t764, t765) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk240::<F>(t760, t762, t206, t262);
        let t766 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk241::<F>(t78);
    (t751, t752, t754, t755, t757, t759, t760, t762, t764, t765, t766)
}
