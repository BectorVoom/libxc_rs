//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta110 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk643;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk644;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk645;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk646;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta110<F: Float>(t157: F, t2516: F, t153: F, t145: F, t2447: F, t185: F, t193: F, t2373: F, t2377: F, t2378: F, t2379: F, t2408: F, t2417: F, t2423: F, t2426: F, t2429: F, t2432: F, t2450: F, t201: F, t868: F, t870: F, t2369: F, t2509: F, t2512: F) -> (F, F, F, F, F, F, F, F) {
        let t2517 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk643::<F>(t157, t2516);
        let (t2518, t2519, t2520, t2521) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk644::<F>(t153, t2517, t145, t2447, t185, t193, t2373, t2377, t2378, t2379, t2408, t2417, t2423, t2426, t2429, t2432, t2450);
        let t2522 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk645::<F>(t193, t201);
        let (t2523, t2528) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk646::<F>(t868, t870, t2369, t2509, t2512);
    (t2517, t2518, t2519, t2520, t2521, t2522, t2523, t2528)
}
