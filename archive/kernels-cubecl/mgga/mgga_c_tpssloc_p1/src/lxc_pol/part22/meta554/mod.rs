//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta554 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2054;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta554<F: Float>(t40889: F, t68: F, t852: F, t9971: F, t233: F, t9970: F, t2632: F, t10021: F, t812: F, t841: F, t849: F, t23076: F, t241: F, t67: F) -> (F, F, F, F, F, F, F) {
        let (t40890, t40917, t40931, t40933, t40965, t40966, t40971) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2054::<F>(t40889, t68, t852, t9971, t233, t9970, t2632, t10021, t812, t841, t849, t23076, t241, t67);
    (t40890, t40917, t40931, t40933, t40965, t40966, t40971)
}
