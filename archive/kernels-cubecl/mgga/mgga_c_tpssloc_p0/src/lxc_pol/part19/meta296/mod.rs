//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta296 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1078;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta296<F: Float>(t13257: F, t812: F, t242: F, t9972: F, t820: F, t9645: F, t4290: F, t808: F, t68: F, t9971: F, t226: F, t4280: F) -> (F, F, F, F, F, F) {
        let (t13258, t13262, t13350, t13390, t13397, t13453) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1078::<F>(t13257, t812, t242, t9972, t820, t9645, t4290, t808, t68, t9971, t226, t4280);
    (t13258, t13262, t13350, t13390, t13397, t13453)
}
