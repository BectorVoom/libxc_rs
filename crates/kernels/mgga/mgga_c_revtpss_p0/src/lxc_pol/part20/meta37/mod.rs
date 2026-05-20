//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta37 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk264;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk265;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk266;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta37<F: Float>(t158: F, t750: F, t162: F, t716: F, t187: F, t192: F, t72: F, t186: F, t675: F, t685: F, t177: F, t738: F, t744: F, t745: F) -> (F, F, F, F, F, F, F, F) {
        let (t751, t752, t754, t755, t757) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk264::<F>(t158, t750, t162, t716, t187, t192, t72, t186, t675, t685);
        let (t759, t760) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk265::<F>(t755, t757, t177, t192);
        let t762 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk266::<F>(t738, t744, t745);
    (t751, t752, t754, t755, t757, t759, t760, t762)
}
