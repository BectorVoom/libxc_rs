//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta322 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1083;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1084;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1085;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta322<F: Float>(t21812: F, t21815: F, t21829: F, t21832: F, t21835: F, t21956: F, t21958: F, t21960: F, t21963: F, t22224: F, t22226: F, t11292: F, t21906: F, t3403: F, t1164: F, t1147: F, t1156: F, t21938: F, t11282: F, t11285: F, t4869: F, t6102: F, t21726: F, t21728: F, t21730: F, t21732: F, t21897: F, t21901: F, t21990: F, t21993: F) -> (F, F, F, F, F, F, F, F) {
        let (t22227, t22228) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1083::<F>(t21812, t21815, t21829, t21832, t21835, t21956, t21958, t21960, t21963, t22224, t22226, t11292, t21906);
        let (t22229, t22231, t22233, t22235, t22237, t22239, t22241, t22242) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1084::<F>(t22228, t3403, t1164, t1147, t1156, t21938, t11282, t21906, t11285, t4869, t6102, t21726, t21728, t21730, t21732, t21897, t21901, t21990, t21993);
        let t22243 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1085::<F>(t22227, t22242);
    (t22229, t22231, t22233, t22235, t22237, t22239, t22241, t22243)
}
