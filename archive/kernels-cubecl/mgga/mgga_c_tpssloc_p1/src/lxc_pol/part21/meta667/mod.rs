//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta667 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2468;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta667<F: Float>(t11889: F, t3493: F, t3502: F, t42341: F, t44696: F, t23508: F, t3508: F, t1209: F, t1174: F, t3551: F, t698: F, t3242: F, t415: F) -> (F, F, F, F, F, F) {
        let (t44741, t44753, t44754, t44785, t44811, t44827) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2468::<F>(t11889, t3493, t3502, t42341, t44696, t23508, t3508, t1209, t1174, t3551, t698, t3242, t415);
    (t44741, t44753, t44754, t44785, t44811, t44827)
}
