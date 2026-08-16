//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta81 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk471;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk472;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk473;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk474;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta81(t2475: f64, t159: f64, t167: f64, t2461: f64, t676: f64, t682: f64, t268: f64, t703: f64, t739: f64, t172: f64, t2368: f64, t2369: f64, t746: f64, t2388: f64, t2391: f64, t2394: f64, t2398: f64, t2400: f64, t2403: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2476, t2477, t2478, t2479) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk471(t2475, t159, t167);
        let (t2480, t2483, t2486) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk472(t2461, t2479, t676, t682, t268, t703);
        let (t2490, t2494, t2495, t2504) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk473(t676, t739, t172, t2368, t2369, t746, t2388, t2391, t2394, t2398, t2400, t2403);
        let t2505 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk474(t2504, t746);
    (t2476, t2477, t2478, t2479, t2480, t2483, t2486, t2490, t2494, t2495, t2504, t2505)
}
