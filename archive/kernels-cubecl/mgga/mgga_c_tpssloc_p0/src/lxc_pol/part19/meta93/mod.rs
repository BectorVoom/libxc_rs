//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta93 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk524;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk525;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk526;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk527;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk528;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk529;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk530;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta93<F: Float>(t2732: F, t829: F, t2679: F, t860: F, t2684: F, t235: F, t2710: F, t226: F, t255: F, t2613: F, t2617: F, t2729: F, t808: F, t812: F, t861: F, t863: F, t858: F, t259: F, t2592: F, t2594: F, t2597: F, t2711: F, t2713: F, t2720: F, t855: F, t866: F, t868: F, t261: F, t193: F, t202: F, t2486: F, t2522: F, t2523: F, t2530: F, t2533: F, t2537: F, t2539: F, t2553: F, t2654: F, t2657: F, t2661: F, t2665: F, t766: F, t776: F, t870: F, t2521: F, t1878: F, t268: F, t271: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2733, t2736, t2738, t2740, t2742) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk524::<F>(t2732, t829, t2679, t860, t2684, t235, t2710, t226, t255, t2613, t2617, t2729, t808, t812, t861, t863);
        let (t2743, t2745) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk525::<F>(t2742, t858, t259, t2592, t2594, t2597, t2711, t2713, t2720, t855, t866);
        let t2749 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk526::<F>(t868);
        let (t2751, t2752) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk527::<F>(t261);
        let t2755 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk528::<F>(t193, t202, t2486, t2522, t2523, t2530, t2533, t2537, t2539, t2553, t2654, t2657, t2661, t2665, t2745, t2749, t2752, t766, t776, t870);
        let t2756 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk529::<F>(t2521, t2755);
        let t2764 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk530::<F>(t1878, t268, t271);
    (t2733, t2736, t2738, t2740, t2742, t2743, t2745, t2749, t2751, t2752, t2756, t2764)
}
