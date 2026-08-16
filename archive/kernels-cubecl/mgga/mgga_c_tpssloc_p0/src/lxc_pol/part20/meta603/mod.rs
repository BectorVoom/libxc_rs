//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta603 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2183;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2184;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta603<F: Float>(t1174: F, t11765: F, t135: F, t3551: F, t698: F, t3242: F, t415: F, t42341: F, t44696: F, t42344: F, t483: F, t1210: F, t3561: F, t11738: F, t11739: F, t248: F, t3570: F, t10471: F, t44690: F, t11727: F, t44722: F, t478: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t44803, t44811, t44827, t44833, t44834, t44836) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2183::<F>(t1174, t11765, t135, t3551, t698, t3242, t415, t42341, t44696, t42344, t483, t1210);
        let (t44847, t44851, t44857, t44858, t44863) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2184::<F>(t1174, t3561, t698, t11738, t11739, t248, t3570, t10471, t44690, t11727, t44722, t44833, t44834, t478);
    (t44803, t44811, t44827, t44833, t44834, t44836, t44847, t44851, t44857, t44858, t44863)
}
