//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta178 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk804;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk805;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta178<F: Float>(t9798: F, t9860: F, t157: F, t153: F, t181: F, t686: F, t781: F, t756: F, t2371: F, t677: F, t2374: F, t2535: F, t2528: F, t2509: F, t745: F, t9843: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9861, t9862, t9863, t9874, t9876, t9882, t9884, t9885) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk804::<F>(t9798, t9860, t157, t153, t181, t686, t781, t756, t2371, t677, t2374, t2535);
        let (t9887, t9888, t9890, t9892) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk805::<F>(t2374, t9885, t2528, t677, t2509, t745, t9843);
    (t9861, t9862, t9863, t9874, t9876, t9882, t9884, t9885, t9887, t9888, t9890, t9892)
}
