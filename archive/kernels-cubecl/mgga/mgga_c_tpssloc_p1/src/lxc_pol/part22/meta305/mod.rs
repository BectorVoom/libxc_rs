//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta305 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1472;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1473;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1474;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1475;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1476;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta305<F: Float>(t14507: F, t3038: F, t225: F, t4658: F, t4553: F, t4559: F, t4555: F, t14506: F, t3199: F, t3185: F, t1057: F, t14205: F, t1654: F, t2394: F, t4734: F, t690: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t14511 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1472::<F>(t14507, t3038);
        let (t14529, t14545, t14552, t14555, t14608) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1473::<F>(t225, t4658, t4553, t4559, t4555, t14506, t3199);
        let t14618 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1474::<F>(t14506, t3185);
        let (t14651, t14702) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1475::<F>(t1057, t14205, t1654, t2394);
        let t14704 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1476::<F>(t4734, t690);
    (t14511, t14529, t14545, t14552, t14555, t14608, t14618, t14651, t14702, t14704)
}
