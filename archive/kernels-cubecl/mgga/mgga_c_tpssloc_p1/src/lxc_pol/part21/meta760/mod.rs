//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta760 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2634;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta760<F: Float>(t5154: F, t9722: F, t2221: F, t5166: F, t1788: F, t9216: F, t9218: F, t9494: F, t15892: F, t2535: F, t2528: F, t15971: F, t588: F) -> (F, F, F, F, F, F, F, F) {
        let (t54451, t54456, t54460, t54462, t54467, t54469, t54471, t54477) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2634::<F>(t5154, t9722, t2221, t5166, t1788, t9216, t9218, t9494, t15892, t2535, t2528, t15971, t588);
    (t54451, t54456, t54460, t54462, t54467, t54469, t54471, t54477)
}
