//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta225 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk873;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta225<F: Float>(t1725: F, t698: F, t1174: F, t5168: F, t588: F, t592: F, t2528: F, t5154: F, t2535: F, t118: F, t1787: F) -> (F, F, F, F, F, F, F) {
        let (t15753, t15754, t15875, t15877, t15890, t15895, t15908) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk873::<F>(t1725, t698, t1174, t5168, t588, t592, t2528, t5154, t2535, t118, t1787);
    (t15753, t15754, t15875, t15877, t15890, t15895, t15908)
}
