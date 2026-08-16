//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta101 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk687;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk688;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk689;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk690;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk691;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk692;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk693;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk694;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta101<F: Float>(t244: F, t248: F, t2691: F, t238: F, t835: F, t841: F, t812: F, t849: F, t1891: F, t241: F, t67: F, t225: F, t853: F, t257: F, t856: F, t68: F, t252: F, t2627: F, t814: F, t852: F, t261: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t2693 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk687::<F>(t244, t248, t2691);
        let (t2695, t2696) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk688::<F>(t238, t2693, t835, t841);
        let t2697 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk689::<F>(t2696, t812);
        let (t2698, t2701) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk690::<F>(t2697, t849, t1891, t241, t67);
        let t2713 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk691::<F>(t225, t853);
        let t2718 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk692::<F>(t257, t856, t68);
        let (t2728, t2732) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk693::<F>(t252, t2627, t814, t852);
        let (t2751, t2752) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk694::<F>(t261);
    (t2693, t2695, t2696, t2697, t2698, t2701, t2713, t2718, t2728, t2732, t2751, t2752)
}
