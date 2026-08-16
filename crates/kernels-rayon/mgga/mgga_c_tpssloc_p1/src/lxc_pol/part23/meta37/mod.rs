//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta37 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk264;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk265;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk266;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk267;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk268;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk269;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta37(t207: f64, t792: f64, t795: f64, t154: f64, t782: f64, t222: f64, t226: f64, t68: f64, t233: f64, t236: f64, t240: f64, t241: f64, t244: f64, t67: f64, t120: f64, t246: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t797, t801, t803, t812) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk264(t207, t792, t795, t154, t782, t222, t226, t68);
        let (t813, t814) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk265(t233);
        let t815 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk266(t236, t814);
        let (t816, t817) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk267(t240, t815, t812);
        let t819 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk268(t241, t244, t67);
        let t820 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk269(t120, t246);
    (t797, t801, t803, t812, t813, t814, t815, t816, t817, t819, t820)
}
