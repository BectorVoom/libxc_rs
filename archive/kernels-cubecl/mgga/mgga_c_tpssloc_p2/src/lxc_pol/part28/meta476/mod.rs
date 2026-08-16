//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta476 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1689;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1690;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta476<F: Float>(t25373: F, t25374: F, t1530: F, t606: F, t25: F, t4303: F, t1408: F, t776: F, t868: F, t28: F, t870: F, t4255: F, t16596: F, t23788: F, t1081: F, t1484: F, t4119: F, t25365: F, t10143: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25375, t25377, t25381, t25385, t25392, t25891, t25892) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1689::<F>(t25373, t25374, t1530, t606, t25, t4303, t1408, t776, t868, t28, t870, t4255);
        let (t25898, t25901, t25905, t25921, t25927) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1690::<F>(t16596, t23788, t1081, t1484, t28, t4119, t25365, t10143);
    (t25375, t25377, t25381, t25385, t25392, t25891, t25892, t25898, t25901, t25905, t25921, t25927)
}
