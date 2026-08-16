//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta465 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1931;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta465<F: Float>(t1653: F, t3509: F, t3578: F, t3516: F, t1742: F, t478: F, t3068: F, t1244: F) -> (F, F, F, F, F, F, F) {
        let (t15559, t15560, t15563, t15564, t15567, t15568, t15569) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1931::<F>(t1653, t3509, t3578, t3516, t1742, t478, t3068, t1244);
    (t15559, t15560, t15563, t15564, t15567, t15568, t15569)
}
