//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta173 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1069;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1070;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1071;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1072;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1073;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1074;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta173<F: Float>(t119: F, t4119: F, t210: F, t225: F, t4142: F, t237: F, t1499: F, t68: F, t816: F, t1500: F, t838: F, t842: F, t242: F, t2628: F, t812: F, t244: F, t67: F, t246: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4158, t4159, t4162) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1069::<F>(t119, t4119, t210, t225, t4142);
        let (t4163, t4166) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1070::<F>(t237, t4162, t1499, t68);
        let t4167 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1071::<F>(t4166, t816);
        let (t4170, t4172) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1072::<F>(t1500, t838, t4166, t842);
        let (t4177, t4178) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1073::<F>(t242, t2628, t812);
        let (t4179, t4180) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1074::<F>(t244, t67, t246);
    (t4158, t4159, t4162, t4163, t4166, t4167, t4170, t4172, t4177, t4178, t4179, t4180)
}
