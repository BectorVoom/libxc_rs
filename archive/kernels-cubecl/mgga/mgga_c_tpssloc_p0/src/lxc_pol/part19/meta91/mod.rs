//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta91 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk519;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk520;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk521;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk522;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta91<F: Float>(t241: F, t2690: F, t244: F, t248: F, t238: F, t835: F, t841: F, t812: F, t849: F, t1891: F, t67: F, t2379: F, t820: F, t2553: F, t847: F, t249: F, t2571: F, t2602: F, t2603: F, t2606: F, t2610: F, t2614: F, t2618: F, t2621: F, t2623: F, t2630: F, t2635: F, t2640: F, t2643: F, t2649: F, t2681: F, t2686: F, t787: F, t817: F, t831: F, t843: F) -> (F, F, F, F, F, F, F, F) {
        let (t2691, t2693, t2695, t2696, t2697) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk519::<F>(t241, t2690, t244, t248, t238, t835, t841, t812);
        let (t2698, t2701, t2703) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk520::<F>(t2697, t849, t1891, t241, t67, t2379, t820);
        let t2707 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk521::<F>(t2553, t820, t847);
        let t2710 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk522::<F>(t249, t2571, t2602, t2603, t2606, t2610, t2614, t2618, t2621, t2623, t2630, t2635, t2640, t2643, t2649, t2681, t2686, t2695, t2698, t2703, t2707, t787, t817, t831, t843, t849);
    (t2691, t2693, t2696, t2697, t2701, t2703, t2707, t2710)
}
