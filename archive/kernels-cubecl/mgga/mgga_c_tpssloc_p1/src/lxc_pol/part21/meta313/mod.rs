//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta313 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1675;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1676;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta313<F: Float>(t248: F, t3509: F, t3570: F, t3506: F, t135: F, t3561: F, t1174: F, t3247: F, t415: F, t61: F, t121: F, t3584: F, t3243: F, t1227: F, t1229: F, t676: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11745, t11746, t11754, t11755, t11778, t11779, t11784) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1675::<F>(t248, t3509, t3570, t3506, t135, t3561, t1174, t3247, t415, t61, t121, t3584);
        let (t11786, t11787, t11789) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1676::<F>(t11784, t248, t3243, t1227, t1229, t676);
    (t11745, t11746, t11754, t11755, t11778, t11779, t11784, t11786, t11787, t11789)
}
