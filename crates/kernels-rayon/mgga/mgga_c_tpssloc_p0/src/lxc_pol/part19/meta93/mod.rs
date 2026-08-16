//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta93 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk524;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk525;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk526;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk527;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk528;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk529;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk530;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta93(t2732: f64, t829: f64, t2679: f64, t860: f64, t2684: f64, t235: f64, t2710: f64, t226: f64, t255: f64, t2613: f64, t2617: f64, t2729: f64, t808: f64, t812: f64, t861: f64, t863: f64, t858: f64, t259: f64, t2592: f64, t2594: f64, t2597: f64, t2711: f64, t2713: f64, t2720: f64, t855: f64, t866: f64, t868: f64, t261: f64, t193: f64, t202: f64, t2486: f64, t2522: f64, t2523: f64, t2530: f64, t2533: f64, t2537: f64, t2539: f64, t2553: f64, t2654: f64, t2657: f64, t2661: f64, t2665: f64, t766: f64, t776: f64, t870: f64, t2521: f64, t1878: f64, t268: f64, t271: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2733, t2736, t2738, t2740, t2742) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk524(t2732, t829, t2679, t860, t2684, t235, t2710, t226, t255, t2613, t2617, t2729, t808, t812, t861, t863);
        let (t2743, t2745) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk525(t2742, t858, t259, t2592, t2594, t2597, t2711, t2713, t2720, t855, t866);
        let t2749 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk526(t868);
        let (t2751, t2752) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk527(t261);
        let t2755 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk528(t193, t202, t2486, t2522, t2523, t2530, t2533, t2537, t2539, t2553, t2654, t2657, t2661, t2665, t2745, t2749, t2752, t766, t776, t870);
        let t2756 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk529(t2521, t2755);
        let t2764 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk530(t1878, t268, t271);
    (t2733, t2736, t2738, t2740, t2742, t2743, t2745, t2749, t2751, t2752, t2756, t2764)
}
