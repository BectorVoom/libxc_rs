//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta109 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk752;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk753;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk754;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk755;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk756;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk757;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk758;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk759;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta109(t238: f64, t2693: f64, t835: f64, t841: f64, t812: f64, t849: f64, t1891: f64, t241: f64, t67: f64, t2379: f64, t820: f64, t2553: f64, t847: f64, t249: f64, t2571: f64, t2602: f64, t2603: f64, t2606: f64, t2610: f64, t2614: f64, t2618: f64, t2621: f64, t2623: f64, t2630: f64, t2635: f64, t2640: f64, t2643: f64, t2649: f64, t2681: f64, t2686: f64, t787: f64, t817: f64, t831: f64, t843: f64, t218: f64, t225: f64, t853: f64, t257: f64, t856: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2695, t2696) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk752(t238, t2693, t835, t841);
        let t2697 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk753(t2696, t812);
        let (t2698, t2700, t2701) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk754(t2697, t849, t1891, t241, t67);
        let t2703 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk755(t2379, t2701, t820);
        let t2707 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk756(t2553, t820, t847);
        let t2710 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk757(t249, t2571, t2602, t2603, t2606, t2610, t2614, t2618, t2621, t2623, t2630, t2635, t2640, t2643, t2649, t2681, t2686, t2695, t2698, t2703, t2707, t787, t817, t831, t843, t849);
        let (t2711, t2713) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk758(t218, t2710, t225, t853);
        let t2718 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk759(t257, t856, t68);
    (t2695, t2696, t2697, t2698, t2700, t2701, t2703, t2707, t2710, t2711, t2713, t2718)
}
