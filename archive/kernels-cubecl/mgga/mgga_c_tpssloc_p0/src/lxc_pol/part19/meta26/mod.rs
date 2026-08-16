//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta26 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk199;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk200;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk201;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk202;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk203;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk204;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk205;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta26<F: Float>(t25: F, t28: F, t17: F, t522: F, t182: F, t521: F, t514: F, t194: F, t517: F, zeta_threshold: F, t154: F, t205: F, t215: F, t131: F, t221: F, t225: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t523, t525, t526, t528, t531) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk199::<F>(t25, t28, t17, t522, t182, t521, t514, t194, t517, zeta_threshold);
        let t532 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk200::<F>(t531);
        let t533 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk201::<F>(t531, t532);
        let (t534, t535) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk202::<F>(t532, t154);
        let t539 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk203::<F>(t205, t215, t535);
        let t541 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk204::<F>(t131, t534, t221);
        let t544 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk205::<F>(t225, t539);
    (t523, t525, t526, t528, t531, t532, t533, t534, t535, t539, t541, t544)
}
