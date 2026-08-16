//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta245 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1358;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1359;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1360;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1361;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta245<F: Float>(t2476: F, t676: F, t2504: F, t2512: F, t745: F, t747: F, t2405: F, t2411: F, t2414: F, t701: F, t118: F, t142: F, t9697: F, t181: F, t2454: F, t2459: F, t2460: F, t2462: F, t2471: F, t2472: F, t2477: F, t2479: F, t2480: F, t2490: F, t2494: F, t2495: F, t2505: F, t2510: F, t2513: F, t268: F, t730: F, t732: F, t9799: F, t9803: F, t9810: F, t9814: F, t9820: F, t9824: F, t9798: F, t157: F) -> (F, F, F, F, F, F, F, F) {
        let (t9828, t9843, t9844, t9847, t9853) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1358::<F>(t2476, t676, t2504, t2512, t745, t747, t2405, t2411, t2414, t701);
        let t9859 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1359::<F>(t118, t142, t9697);
        let t9860 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1360::<F>(t118, t181, t2454, t2459, t2460, t2462, t2471, t2472, t2477, t2479, t2480, t2490, t2494, t2495, t2505, t2510, t2513, t268, t676, t730, t732, t747, t9697, t9799, t9803, t9810, t9814, t9820, t9824, t9828, t9844, t9847, t9853, t9859);
        let (t9861, t9862) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1361::<F>(t9798, t9860, t157);
    (t9828, t9843, t9844, t9847, t9853, t9859, t9861, t9862)
}
