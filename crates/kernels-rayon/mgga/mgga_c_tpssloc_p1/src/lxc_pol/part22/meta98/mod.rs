//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta98 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk670;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk671;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk672;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk673;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta98(t118: f64, t776: f64, t794: f64, t2576: f64, t59: f64, t835: f64, t154: f64, t116: f64, t206: f64, t212: f64, t225: f64, t799: f64, t2559: f64, t222: f64, t2563: f64, t805: f64, t68: f64, t808: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2578, t2579, t2585) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk670(t118, t776, t794, t2576, t59, t835);
        let t2586 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk671(t154, t2585);
        let (t2588, t2590, t2597) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk672(t116, t206, t212, t2586, t225, t799);
        let (t2600, t2602, t2603, t2617) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk673(t154, t2559, t222, t2563, t805, t68, t808);
    (t2578, t2579, t2585, t2586, t2588, t2590, t2597, t2600, t2602, t2603, t2617)
}
