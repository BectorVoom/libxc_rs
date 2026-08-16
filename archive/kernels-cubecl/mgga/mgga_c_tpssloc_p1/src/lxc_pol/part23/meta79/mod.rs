//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta79 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk462;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk463;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk464;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk465;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta79<F: Float>(t2475: F, t159: F, t167: F, t2461: F, t676: F, t682: F, t268: F, t703: F, t739: F, t172: F, t2368: F, t2369: F, t746: F, t2388: F, t2391: F, t2394: F, t2398: F, t2400: F, t2403: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2476, t2477, t2478, t2479) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk462::<F>(t2475, t159, t167);
        let (t2480, t2483, t2486) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk463::<F>(t2461, t2479, t676, t682, t268, t703);
        let (t2490, t2494, t2495, t2504) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk464::<F>(t676, t739, t172, t2368, t2369, t746, t2388, t2391, t2394, t2398, t2400, t2403);
        let t2505 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk465::<F>(t2504, t746);
    (t2476, t2477, t2478, t2479, t2480, t2483, t2486, t2490, t2494, t2495, t2504, t2505)
}
