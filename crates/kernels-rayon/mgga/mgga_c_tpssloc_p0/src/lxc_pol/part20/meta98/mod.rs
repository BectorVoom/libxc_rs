//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta98 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk666;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk667;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk668;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk669;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta98(t40: f64, t2244: f64, t2250: f64, t2433: f64, t73: f64, t197: f64, zeta_threshold: f64, t52: f64, t76: f64, t157: f64, t182: f64, t676: f64, t724: f64, t164: f64, t723: f64, t159: f64, t730: f64, t731: f64, t2388: f64, t2391: f64, t2394: f64, t2398: f64, t2400: f64, t2403: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2439, t2440) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk666(t40, t2244, t2250, t2433, t73, t197, zeta_threshold);
        let (t2447, t2448, t2450, t2454, t2458, t2459, t2460) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk667(t52, t2244, t2250, t2440, t76, t2439, t157, t182, t676, t724, t164, t723, t159, zeta_threshold);
        let t2461 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk668(t730);
        let (t2462, t2471) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk669(t2461, t731, t2388, t2391, t2394, t2398, t2400, t2403);
    (t2440, t2447, t2448, t2450, t2454, t2458, t2459, t2460, t2461, t2462, t2471)
}
