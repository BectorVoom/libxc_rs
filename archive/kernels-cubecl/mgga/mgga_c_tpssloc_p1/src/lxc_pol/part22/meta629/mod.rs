//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta629 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2164;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta629<F: Float>(t2225: F, t5166: F, t15921: F, t592: F, t17: F, t2516: F, t5151: F, t1787: F, t9861: F, t15971: F, t2221: F, t5168: F) -> (F, F, F, F, F, F) {
        let (t54401, t54403, t54409, t54411, t54412, t54428) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2164::<F>(t2225, t5166, t15921, t592, t17, t2516, t5151, t1787, t9861, t15971, t2221, t5168);
    (t54401, t54403, t54409, t54411, t54412, t54428)
}
