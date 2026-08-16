//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta96 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk627;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk628;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk629;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk630;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk631;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk632;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk633;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta96<F: Float>(t477: F, t483: F, sigma2: F, t471: F, t2128: F, t2134: F, t2136: F, t488: F, t466: F, t225: F, t491: F, t462: F, t493: F, t2121: F, t470: F, t1241: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t2139, t2140) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk627::<F>(t477, t483, sigma2);
        let (t2141, t2144) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk628::<F>(t2140, t471, t2128, t2134, t2136, t488);
        let (t2145, t2147) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk629::<F>(t2144, t466, t225, t477);
        let t2148 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk630::<F>(t2147, t491);
        let (t2149, t2152) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk631::<F>(t2148, t462, t2144, t493);
        let t2154 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk632::<F>(t2121, t2149, t2152, t470);
        let t2155 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk633::<F>(t1241, t2154);
    (t2139, t2140, t2141, t2144, t2145, t2147, t2148, t2149, t2152, t2154, t2155)
}
