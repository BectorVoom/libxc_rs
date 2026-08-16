//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta357 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1588;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1589;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1590;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1591;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1592;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1593;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1594;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta357<F: Float>(t2770: F, t5398: F, t607: F, t2768: F, t123: F, t2775: F, t882: F, t16558: F, t883: F, t10556: F, t10608: F, t13598: F, t14352: F, t14353: F, t14354: F, t17149: F, t17154: F, t17159: F, t17163: F, t17165: F, t17169: F, t17173: F, t17175: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17177, t17178) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1588::<F>(t2770, t5398, t607);
        let (t17179, t17180) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1589::<F>(t17178, t2768, t123);
        let (t17182, t17183) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1590::<F>(t2775, t5398, t607);
        let (t17184, t17185) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1591::<F>(t17183, t882, t123);
        let t17187 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1592::<F>(t16558, t883);
        let (t17188, t17189) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1593::<F>(t17187, t882, t123);
        let t17191 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1594::<F>(t10556, t10608, t13598, t14352, t14353, t14354, t17149, t17154, t17159, t17163, t17165, t17169, t17173, t17175, t17180, t17185, t17189);
    (t17177, t17178, t17179, t17180, t17182, t17183, t17184, t17185, t17187, t17188, t17189, t17191)
}
