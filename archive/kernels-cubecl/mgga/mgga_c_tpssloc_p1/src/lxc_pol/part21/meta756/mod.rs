//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta756 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2630;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta756<F: Float>(t12283: F, t16308: F, t1824: F, t3791: F, t12300: F, t5289: F, t16208: F, t3799: F, t1788: F, t9212: F, t9214: F, t2223: F, t5168: F) -> (F, F, F, F, F, F, F) {
        let (t54237, t54258, t54293, t54295, t54312, t54314, t54316) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2630::<F>(t12283, t16308, t1824, t3791, t12300, t5289, t16208, t3799, t1788, t9212, t9214, t2223, t5168);
    (t54237, t54258, t54293, t54295, t54312, t54314, t54316)
}
