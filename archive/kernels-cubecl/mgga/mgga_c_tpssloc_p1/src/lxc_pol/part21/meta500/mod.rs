//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta500 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2130;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2131;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2132;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2133;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2134;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2135;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2136;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta500<F: Float>(t5686: F, t690: F, t2770: F, t5398: F, t607: F, t2768: F, t123: F, t2775: F, t882: F, t16558: F, t883: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t17175 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2130::<F>(t5686, t690);
        let (t17177, t17178) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2131::<F>(t2770, t5398, t607);
        let (t17179, t17180) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2132::<F>(t17178, t2768, t123);
        let (t17182, t17183) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2133::<F>(t2775, t5398, t607);
        let (t17184, t17185) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2134::<F>(t17183, t882, t123);
        let t17187 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2135::<F>(t16558, t883);
        let (t17188, t17189) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2136::<F>(t17187, t882, t123);
    (t17175, t17177, t17178, t17179, t17180, t17182, t17183, t17184, t17185, t17187, t17188, t17189)
}
