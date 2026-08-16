//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta326 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1090;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta326<F: Float>(t11915: F, t22348: F, t1734: F, t1932: F, t475: F, t6260: F, t11883: F, t11889: F, t1751: F, t6224: F, t3612: F, t6218: F) -> (F, F, F, F, F, F, F, F) {
        let (t22349, t22354, t22355, t22358, t22361, t22364, t22365, t22368) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1090::<F>(t11915, t22348, t1734, t1932, t475, t6260, t11883, t11889, t1751, t6224, t3612, t6218);
    (t22349, t22354, t22355, t22358, t22361, t22364, t22365, t22368)
}
