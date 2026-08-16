//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta646 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2067;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2068;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta646<F: Float>(t26447: F, t90607: F, t90787: F, t22751: F, t26397: F, t22892: F, t22893: F, t26396: F, t26384: F, t26388: F, t7733: F, t81186: F, t5318: F, t552: F, t5187: F, t562: F, t26392: F, t80670: F, t22705: F, t26422: F, t81228: F, t22704: F, t26466: F, t26461: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t90789, t90792, t90795, t90798, t90806, t90807) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2067::<F>(t26447, t90607, t90787, t22751, t26397, t22892, t22893, t26396, t26384, t26388, t7733, t81186);
        let (t90809, t90818, t90837, t90845, t90860, t90864) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2068::<F>(t5318, t552, t5187, t562, t26392, t80670, t22705, t26422, t81228, t22704, t26466, t26461);
    (t90789, t90792, t90795, t90798, t90806, t90807, t90809, t90818, t90837, t90845, t90860, t90864)
}
