//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta641 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2059;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta641<F: Float>(t2240: F, t3967: F, t12571: F, t608: F, t645: F, t7445: F, t26351: F, t6883: F, t22751: F, t26186: F, t26190: F, t26356: F, t6914: F) -> (F, F, F, F, F, F, F) {
        let (t90104, t90114, t90247, t90460, t90469, t90471, t90472) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2059::<F>(t2240, t3967, t12571, t608, t645, t7445, t26351, t6883, t22751, t26186, t26190, t26356, t6914);
    (t90104, t90114, t90247, t90460, t90469, t90471, t90472)
}
