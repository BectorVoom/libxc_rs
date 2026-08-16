//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta186 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk899;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk900;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta186<F: Float>(t1519: F, t798: F, t1496: F, t2563: F, t1495: F, t210: F, t776: F, t119: F, t4119: F, t225: F, t4142: F, t237: F, t1499: F, t68: F) -> (F, F, F, F, F, F, F, F) {
        let (t4149, t4152, t4155, t4158, t4159, t4162) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk899::<F>(t1519, t798, t1496, t2563, t1495, t210, t776, t119, t4119, t225, t4142);
        let (t4163, t4166) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk900::<F>(t237, t4162, t1499, t68);
    (t4149, t4152, t4155, t4158, t4159, t4162, t4163, t4166)
}
