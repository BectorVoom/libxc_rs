//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta276 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1552;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1553;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1554;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1555;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta276<F: Float>(t2374: F, t9882: F, t2535: F, t677: F, t2528: F, t2509: F, t745: F, t9843: F, t761: F, t152: F, t31: F, t2448: F, t67: F, t758: F, t2368: F, t2505: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9884, t9885) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1552::<F>(t2374, t9882, t2535, t677);
        let (t9887, t9888) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1553::<F>(t2374, t9885, t2528, t677);
        let (t9890, t9892) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1554::<F>(t2374, t9888, t2509, t745, t9843);
        let (t9894, t9897, t9901, t9902, t9905) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1555::<F>(t761, t9892, t152, t31, t2448, t67, t758, t2368, t2505, t745);
    (t9884, t9885, t9887, t9888, t9890, t9892, t9894, t9897, t9901, t9902, t9905)
}
