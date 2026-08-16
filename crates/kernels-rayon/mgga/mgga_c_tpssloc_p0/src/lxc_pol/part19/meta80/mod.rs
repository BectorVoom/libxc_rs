//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta80 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk468;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk469;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk470;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta80(t182: f64, t2448: f64, t676: f64, t724: f64, t164: f64, t723: f64, t159: f64, t730: f64, t731: f64, t2388: f64, t2391: f64, t2394: f64, t2398: f64, t2400: f64, t2403: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2450, t2454, t2458, t2459, t2460, t2461) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk468(t182, t2448, t676, t724, t164, t723, t159, t730);
        let (t2462, t2471) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk469(t2461, t731, t2388, t2391, t2394, t2398, t2400, t2403);
        let (t2472, t2475) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk470(t2471, t731, t723);
    (t2450, t2454, t2458, t2459, t2460, t2461, t2462, t2471, t2472, t2475)
}
