//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta40 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk277;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk278;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk279;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk280;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk281;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk282;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk283;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk284;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta40<F: Float>(t233: F, t236: F, t240: F, t812: F, t241: F, t244: F, t67: F, t120: F, t246: F, t225: F, t680: F, t705: F, t710: F, t719: F, t752: F, t755: F, t760: F, t765: F, t68: F, t776: F, t228: F, t230: F, t232: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t813, t814) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk277::<F>(t233);
        let t815 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk278::<F>(t236, t814);
        let (t816, t817) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk279::<F>(t240, t815, t812);
        let t819 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk280::<F>(t241, t244, t67);
        let t820 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk281::<F>(t120, t246);
        let t822 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk282::<F>(t225, t680, t705, t710, t719, t752, t755, t760, t765);
        let (t824, t825, t828) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk283::<F>(t244, t68, t776, t228, t230, t822);
        let t829 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk284::<F>(t232, t828);
    (t813, t814, t815, t816, t817, t819, t820, t822, t824, t825, t828, t829)
}
