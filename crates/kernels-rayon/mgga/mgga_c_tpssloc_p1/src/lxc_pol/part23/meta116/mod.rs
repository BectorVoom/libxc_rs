//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta116 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk598;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk599;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk600;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk601;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk602;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta116(t1020: f64, t4630: f64, t1009: f64, t1603: f64, t1011: f64, t1019: f64, t1040: f64, t1611: f64, t1626: f64, t225: f64, t1057: f64, t193: f64, t336: f64, t1654: f64, t690: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t4631, t4639, t4641) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk598(t1020, t4630, t1009, t1603, t1011, t1019);
        let t4644 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk599(t1040, t1611);
        let (t4660, t4669) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk600(t1626, t225, t1057, t4639);
        let t4700 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk601(t193, t336);
        let t4721 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk602(t1654, t690);
    (t4631, t4641, t4644, t4660, t4669, t4700, t4721)
}
