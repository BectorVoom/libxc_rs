//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta315 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1121;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1122;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1123;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1124;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1125;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1126;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1127;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta315(t28: f64, t3673: f64, t3231: f64, t39109: f64, t11122: f64, t12072: f64, t12075: f64, t3672: f64, t39436: f64, t517: f64, t157: f64, t39434: f64, t182: f64, zeta_threshold: f64, t2405: f64, t2419: f64, t690: f64, t703: f64, t2410: f64, t2414: f64, t701: f64, t268: f64, t682: f64, t781: f64, t204: f64, t2421: f64, t12083: f64, t172: f64, t763: f64, t12451: f64, t12466: f64, t12477: f64, t3734: f64, t39388: f64, t39393: f64, t39397: f64, t39400: f64, t39408: f64, t39411: f64, t5126: f64, t5160: f64, t6999: f64, t2411: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39437, t39443, t39448, t39454, t39456) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1121(t28, t3673, t3231, t39109, t11122, t12072, t12075, t3672, t39436, t517, t157, t39434, t182, zeta_threshold);
        let t39463 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1122(t2405, t2419, t690, t703);
        let t39468 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1123(t2405, t2410, t2414, t690, t701);
        let t39472 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1124(t268, t682, t703, t781);
        let t39476 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1125(t204, t2419, t2421, t268);
        let (t39479, t39480) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1126(t12083, t172, t763, t12451, t12466, t12477, t3734, t39388, t39393, t39397, t39400, t39408, t39411, t39456, t39463, t39468, t39472, t39476, t5126, t5160, t6999);
        let t39483 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1127(t2405, t2411, t2421);
    (t39437, t39443, t39448, t39454, t39456, t39463, t39468, t39472, t39476, t39479, t39480, t39483)
}
