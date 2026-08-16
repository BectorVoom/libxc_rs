//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta73 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk444;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta73<F: Float>(t2319: F, t89: F, t1266: F, t671: F, t107: F, t2281: F, t626: F, t667: F, t106: F, t655: F, t666: F, t614: F, tau0: F) -> (F, F, F, F, F, F, F, F) {
        let (t2320, t2323, t2327, t2328, t2331, t2332, t2333, t2336) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk444::<F>(t2319, t89, t1266, t671, t107, t2281, t626, t667, t106, t655, t666, t614, tau0);
    (t2320, t2323, t2327, t2328, t2331, t2332, t2333, t2336)
}
