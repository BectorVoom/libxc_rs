//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta180 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk828;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk829;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk830;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta180<F: Float>(t9798: F, t9860: F, t157: F, t153: F, t2371: F, t2531: F, t2528: F, t2517: F, t607: F, t707: F, t2652: F, t2663: F, t181: F, t686: F, t781: F, t756: F, t9727: F, t9780: F, t9789: F, t9793: F, t9797: F, t118: F, t753: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9861, t9862, t9863, t9865, t9867, t9868, t9870, t9871) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk828::<F>(t9798, t9860, t157, t153, t2371, t2531, t2528, t2517, t607, t707, t2652, t2663);
        let (t9872, t9874, t9876, t9877) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk829::<F>(t9871, t181, t686, t781, t756, t9727, t9780, t9789, t9793, t9797, t9863, t9865, t9867, t9870);
        let t9879 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk830::<F>(t118, t753);
    (t9861, t9862, t9863, t9865, t9867, t9868, t9870, t9872, t9874, t9876, t9877, t9879)
}
