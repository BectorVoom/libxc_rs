//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta35 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk253;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk254;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk255;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk256;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk257;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk258;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta35<F: Float>(t510: F, t671: F, t3: F, t60: F, t120: F, t118: F, t142: F, t138: F, t125: F, t126: F, t67: F, t117: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t672 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk253::<F>(t510, t671);
        let t676 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk254::<F>(t3, t60);
        let t677 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk255::<F>(t120, t676);
        let t680 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk256::<F>(t118, t142, t677);
        let (t681, t682, t683, t685, t686) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk257::<F>(t138, t125, t126, t67, t117, t120);
        let (t687, t688, t690) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk258::<F>(t676, t686, t685, t118, t677);
    (t672, t676, t677, t680, t681, t682, t683, t685, t686, t687, t688, t690)
}
