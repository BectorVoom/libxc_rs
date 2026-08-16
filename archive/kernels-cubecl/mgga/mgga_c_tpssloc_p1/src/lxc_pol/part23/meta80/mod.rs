//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta80 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk466;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk467;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk468;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk469;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk470;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk471;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk472;
use chunk7::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk473;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta80<F: Float>(t738: F, t172: F, t180: F, t2369: F, t118: F, t168: F, t181: F, t2393: F, t2408: F, t2417: F, t2423: F, t2426: F, t2454: F, t2460: F, t2462: F, t2472: F, t2477: F, t2480: F, t2486: F, t2490: F, t2494: F, t2495: F, t2505: F, t268: F, t725: F, t732: F, t740: F, t747: F, t157: F, t153: F, t193: F, t201: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t2508 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk466::<F>(t738);
        let t2509 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk467::<F>(t2508);
        let t2510 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk468::<F>(t172, t2509);
        let (t2511, t2512) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk469::<F>(t180);
        let (t2513, t2516) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk470::<F>(t2369, t2512, t118, t168, t181, t2393, t2408, t2417, t2423, t2426, t2454, t2460, t2462, t2472, t2477, t2480, t2486, t2490, t2494, t2495, t2505, t2510, t268, t725, t732, t740, t747);
        let t2517 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk471::<F>(t157, t2516);
        let (t2518, t2522) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk472::<F>(t153, t2517, t193, t201);
        let (t2527, t2528) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk473::<F>(t2369, t2509, t2512);
    (t2508, t2509, t2510, t2511, t2512, t2513, t2516, t2517, t2518, t2522, t2527, t2528)
}
