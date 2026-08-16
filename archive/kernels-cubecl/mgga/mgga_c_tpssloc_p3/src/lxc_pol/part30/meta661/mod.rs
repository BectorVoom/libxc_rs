//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta661 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2083;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta661<F: Float>(t22751: F, t26397: F, t22892: F, t22893: F, t26396: F, t26384: F, t26388: F, t7733: F, t81186: F, t5318: F, t552: F, t5187: F, t562: F) -> (F, F, F, F, F, F, F) {
        let (t90792, t90795, t90798, t90806, t90807, t90809, t90818) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2083::<F>(t22751, t26397, t22892, t22893, t26396, t26384, t26388, t7733, t81186, t5318, t552, t5187, t562);
    (t90792, t90795, t90798, t90806, t90807, t90809, t90818)
}
