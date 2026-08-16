//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta120 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk717;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk718;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk719;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk720;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk721;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk722;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk723;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta120<F: Float>(t238: F, t2693: F, t835: F, t841: F, t812: F, t849: F, t1891: F, t241: F, t67: F, t2379: F, t820: F, t2553: F, t847: F, t249: F, t2571: F, t2602: F, t2603: F, t2606: F, t2610: F, t2614: F, t2618: F, t2621: F, t2623: F, t2630: F, t2635: F, t2640: F, t2643: F, t2649: F, t2681: F, t2686: F, t787: F, t817: F, t831: F, t843: F, t218: F, t225: F, t853: F, t257: F, t856: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2695, t2696, t2697) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk717::<F>(t238, t2693, t835, t841, t812);
        let (t2698, t2701, t2703) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk718::<F>(t2697, t849, t1891, t241, t67, t2379, t820);
        let t2707 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk719::<F>(t2553, t820, t847);
        let t2710 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk720::<F>(t249, t2571, t2602, t2603, t2606, t2610, t2614, t2618, t2621, t2623, t2630, t2635, t2640, t2643, t2649, t2681, t2686, t2695, t2698, t2703, t2707, t787, t817, t831, t843, t849);
        let (t2711, t2713) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk721::<F>(t218, t2710, t225, t853);
        let t2717 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk722::<F>(t257, t856);
        let t2718 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk723::<F>(t2717, t68);
    (t2695, t2696, t2697, t2698, t2701, t2703, t2707, t2710, t2711, t2713, t2717, t2718)
}
