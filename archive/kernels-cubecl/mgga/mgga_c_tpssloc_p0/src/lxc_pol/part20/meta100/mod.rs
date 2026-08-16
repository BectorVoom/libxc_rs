//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta100 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk673;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk674;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk675;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk676;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk677;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk678;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk679;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta100<F: Float>(t676: F, t739: F, t172: F, t2368: F, t2369: F, t746: F, t2388: F, t2391: F, t2394: F, t2398: F, t2400: F, t2403: F, t738: F, t180: F, t118: F, t168: F, t181: F, t2393: F, t2408: F, t2417: F, t2423: F, t2426: F, t2454: F, t2460: F, t2462: F, t2472: F, t2477: F, t2480: F, t2486: F, t268: F, t725: F, t732: F, t740: F, t747: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2490, t2494, t2495, t2504) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk673::<F>(t676, t739, t172, t2368, t2369, t746, t2388, t2391, t2394, t2398, t2400, t2403);
        let t2505 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk674::<F>(t2504, t746);
        let t2508 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk675::<F>(t738);
        let t2509 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk676::<F>(t2508);
        let t2510 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk677::<F>(t172, t2509);
        let (t2511, t2512) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk678::<F>(t180);
        let (t2513, t2516) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk679::<F>(t2369, t2512, t118, t168, t181, t2393, t2408, t2417, t2423, t2426, t2454, t2460, t2462, t2472, t2477, t2480, t2486, t2490, t2494, t2495, t2505, t2510, t268, t725, t732, t740, t747);
    (t2490, t2494, t2495, t2504, t2505, t2508, t2509, t2510, t2511, t2512, t2513, t2516)
}
