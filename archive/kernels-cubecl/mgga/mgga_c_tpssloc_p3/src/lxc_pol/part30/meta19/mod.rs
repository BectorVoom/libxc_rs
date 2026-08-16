//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta19 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk139;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk140;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk141;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk142;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk143;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk144;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk145;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk146;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta19<F: Float>(t340: F, t60: F, t285: F, t221: F, t339: F, t225: F, t68: F, t336: F, t293: F, t328: F, t330: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t341, t343) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk139::<F>(t340, t60, t285);
        let t344 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk140::<F>(t343);
        let t345 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk141::<F>(t341, t344);
        let (t346, t349) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk142::<F>(t221, t345, t339);
        let t350 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk143::<F>(t221, t341);
        let t353 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk144::<F>(t225, t349);
        let t354 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk145::<F>(t353, t68);
        let (t357, t358, t360) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk146::<F>(t336, t68, t225, t293, t328, t330);
    (t343, t344, t345, t346, t349, t350, t353, t354, t357, t358, t360)
}
