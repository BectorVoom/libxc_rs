//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta236 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1333;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1334;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1335;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta236<F: Float>(t2701: F, t820: F, t9616: F, t120: F, t2678: F, t4180: F, t829: F, t2631: F, t2632: F, t776: F, t2645: F, t2646: F, t815: F, t836: F, t812: F, t2649: F, t2617: F, t2642: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9618, t9621, t9623, t9626, t9627, t9629, t9632) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1333::<F>(t2701, t820, t9616, t120, t2678, t4180, t829, t2631, t2632, t776, t2645);
        let (t9634, t9637, t9638) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1334::<F>(t2646, t4180, t9632, t815, t836, t812);
        let (t9639, t9642) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1335::<F>(t2649, t9638, t2617, t2642);
    (t9618, t9621, t9623, t9626, t9627, t9629, t9632, t9634, t9637, t9638, t9639, t9642)
}
