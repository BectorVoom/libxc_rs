//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta154 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk961;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk962;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk963;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta154(t225: f64, t4552: f64, t68: f64, t369: f64, t1031: f64, t1611: f64, t1036: f64, t1612: f64, t1616: f64, t248: f64, t3101: f64, t1020: f64, t1044: f64, t4347: f64, t1009: f64, t1603: f64, t1011: f64, t1019: f64, t1040: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4615, t4616, t4617, t4622, t4625, t4630) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk961(t225, t4552, t68, t369, t1031, t1611, t1036, t1612, t1616, t248, t3101);
        let (t4631, t4636, t4639, t4640, t4641) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk962(t1020, t4630, t1044, t248, t4347, t1009, t1603, t1011, t1019);
        let t4644 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk963(t1040, t1611);
    (t4615, t4616, t4617, t4622, t4625, t4630, t4631, t4636, t4639, t4640, t4641, t4644)
}
