//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta120 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk610;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk611;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk612;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk613;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta120(t1213: f64, t4997: f64, t1009: f64, t1720: f64, t1011: f64, t1212: f64, t1226: f64, t1730: f64, t1017: f64, t1742: f64, t1210: f64, t1207: f64, t372: f64, t479: f64, t471: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4998, t5000, t5001, t5002) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk610(t1213, t4997, t1009, t1720, t1011, t1212);
        let t5005 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk611(t1226, t1730);
        let (t5018, t5019) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk612(t1017, t1742, t1210, t1207);
        let (t5023, t5024) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk613(t1742, t372, t479, t471);
    (t4998, t5000, t5001, t5002, t5005, t5018, t5019, t5023, t5024)
}
