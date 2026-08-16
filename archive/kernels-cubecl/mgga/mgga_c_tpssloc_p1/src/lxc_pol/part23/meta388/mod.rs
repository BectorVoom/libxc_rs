//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta388 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1192;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta388<F: Float>(t15971: F, t592: F, t2221: F, t5168: F, t2225: F, t5154: F, t9892: F, t9722: F, t1788: F, t9216: F, t9218: F, t9494: F) -> (F, F, F, F, F, F, F, F) {
        let (t54412, t54428, t54432, t54434, t54451, t54460, t54462, t54467) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1192::<F>(t15971, t592, t2221, t5168, t2225, t5154, t9892, t9722, t1788, t9216, t9218, t9494);
    (t54412, t54428, t54432, t54434, t54451, t54460, t54462, t54467)
}
