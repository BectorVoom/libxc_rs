//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta514 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1880;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1881;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1882;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1883;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1884;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta514<F: Float>(t25276: F, t25328: F, t858: F, t23237: F, t7479: F, t6552: F, t4119: F, t6554: F, t6553: F, t23204: F, t23164: F, t225: F, t7511: F, t13042: F, t1912: F, t23249: F, t23252: F, t23254: F, t23262: F, t25230: F, t25233: F, t2597: F, t2713: F, t7517: F, t855: F, t866: F, t25173: F, t25196: F, t25228: F, t870: F, t2752: F, t7540: F, t1530: F, t776: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t25329, t25330, t25338, t25339, t25341, t25342, t25343, t25345, t25346, t25348) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1880::<F>(t25276, t25328, t858, t23237, t7479, t6552, t4119, t6554, t6553, t23204, t23164, t225, t7511);
        let t25351 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1881::<F>(t13042, t1912, t23249, t23252, t23254, t23262, t25230, t25233, t25330, t25339, t25343, t25346, t25348, t2597, t2713, t7517, t855, t866);
        let (t25353, t25354) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1882::<F>(t25173, t25196, t25228, t25351, t870);
        let t25358 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1883::<F>(t2752, t7540);
        let t25365 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1884::<F>(t1530, t776);
    (t25329, t25330, t25338, t25341, t25342, t25345, t25348, t25353, t25354, t25358, t25365)
}
