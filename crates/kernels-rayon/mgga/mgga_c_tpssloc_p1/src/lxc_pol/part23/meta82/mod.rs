//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta82 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk481;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk482;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk483;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk484;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta82(t786: f64, t792: f64, t59: f64, t835: f64, t154: f64, t116: f64, t206: f64, t212: f64, t2559: f64, t222: f64, t233: f64, t813: f64, t236: f64, t240: f64, t812: f64, t232: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2576, t2585, t2586) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk481(t786, t792, t59, t835, t154);
        let (t2588, t2590, t2600, t2602, t2627) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk482(t116, t206, t212, t2586, t154, t2559, t222, t233, t813);
        let t2628 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk483(t236, t2627);
        let (t2629, t2630, t2632) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk484(t240, t2628, t812, t232);
    (t2576, t2585, t2586, t2588, t2590, t2600, t2602, t2627, t2628, t2629, t2630, t2632)
}
