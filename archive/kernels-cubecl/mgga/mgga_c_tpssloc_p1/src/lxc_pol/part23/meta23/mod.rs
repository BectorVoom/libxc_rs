//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta23 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk172;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk173;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk174;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk175;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk176;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk177;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk178;
use chunk7::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk179;
use chunk8::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk180;
use chunk9::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk181;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta23<F: Float>(t457: F, t60: F, t417: F, t221: F, t456: F, t225: F, t68: F, t358: F, t425: F, t453: F, t455: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t458, t460) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk172::<F>(t457, t60, t417);
        let t461 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk173::<F>(t460);
        let t463 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk174::<F>(t458, t461, t221);
        let t466 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk175::<F>(t456, t463);
        let t467 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk176::<F>(t221, t458);
        let t470 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk177::<F>(t225, t466);
        let t471 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk178::<F>(t470, t68);
        let t475 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk179::<F>(t225, t358, t425, t453, t455);
        let (t476, t477, t478) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk180::<F>(t475, sigma2);
        let t479 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk181::<F>(t477, t478);
    (t460, t461, t463, t466, t467, t470, t471, t475, t476, t477, t478, t479)
}
