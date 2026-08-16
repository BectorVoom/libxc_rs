//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta54 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk354;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk355;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk356;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk357;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta54<F: Float>(t636: F, t607: F, t1088: F, t123: F, t1087: F, t423: F, t419: F, t409: F, t410: F, t1086: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t1089 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk354::<F>(t636);
        let t1090 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk355::<F>(t1089, t607);
        let (t1091, t1092, t1094) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk356::<F>(t1088, t1090, t123, t1087);
        let (t1096, t1097, t1098, t1099, t1100, t1102) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk357::<F>(t1094, t423, t419, t409, t410, t1086, t1092);
    (t1089, t1090, t1091, t1092, t1094, t1096, t1097, t1098, t1099, t1100, t1102)
}
