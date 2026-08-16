//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta188 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk823;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta188<F: Float>(t10544: F, t2840: F, t287: F, t275: F, t10294: F, t891: F, t2843: F, t290: F, t10629: F, t315: F, t2884: F, t307: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10636, t10660, t10661, t10675, t10676, t10701, t10702, t10704, t10756, t10770) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk823::<F>(t10544, t2840, t287, t275, t10294, t891, t2843, t290, t10629, t315, t2884, t307);
    (t10636, t10660, t10661, t10675, t10676, t10701, t10702, t10704, t10756, t10770)
}
