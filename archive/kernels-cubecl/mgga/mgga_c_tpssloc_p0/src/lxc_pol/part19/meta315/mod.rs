//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta315 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1121;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1122;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1123;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1124;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1125;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1126;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1127;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta315<F: Float>(t28: F, t3673: F, t3231: F, t39109: F, t11122: F, t12072: F, t12075: F, t3672: F, t39436: F, t517: F, t157: F, t39434: F, t182: F, zeta_threshold: F, t2405: F, t2419: F, t690: F, t703: F, t2410: F, t2414: F, t701: F, t268: F, t682: F, t781: F, t204: F, t2421: F, t12083: F, t172: F, t763: F, t12451: F, t12466: F, t12477: F, t3734: F, t39388: F, t39393: F, t39397: F, t39400: F, t39408: F, t39411: F, t5126: F, t5160: F, t6999: F, t2411: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t39437, t39443, t39448, t39454, t39456) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1121::<F>(t28, t3673, t3231, t39109, t11122, t12072, t12075, t3672, t39436, t517, t157, t39434, t182, zeta_threshold);
        let t39463 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1122::<F>(t2405, t2419, t690, t703);
        let t39468 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1123::<F>(t2405, t2410, t2414, t690, t701);
        let t39472 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1124::<F>(t268, t682, t703, t781);
        let t39476 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1125::<F>(t204, t2419, t2421, t268);
        let (t39479, t39480) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1126::<F>(t12083, t172, t763, t12451, t12466, t12477, t3734, t39388, t39393, t39397, t39400, t39408, t39411, t39456, t39463, t39468, t39472, t39476, t5126, t5160, t6999);
        let t39483 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1127::<F>(t2405, t2411, t2421);
    (t39437, t39443, t39448, t39454, t39456, t39463, t39468, t39472, t39476, t39479, t39480, t39483)
}
