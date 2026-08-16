//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta129 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk715;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk716;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta129<F: Float>(t2880: F, t932: F, t922: F, t302: F, t310: F, t2862: F, t2764: F, t2766: F, t2773: F, t2778: F, t2782: F, t324: F, t938: F, t942: F, t320: F, t941: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2881, t2884, t2885, t2886, t2887, t2888, t2889, t2892, t2897, t2898) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk715::<F>(t2880, t932, t922, t302, t310, t2862, t2764, t2766, t2773, t2778, t2782, t324);
        let (t2900, t2904) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk716::<F>(t938, t942, t320, t941);
    (t2881, t2884, t2885, t2886, t2887, t2888, t2889, t2892, t2897, t2898, t2900, t2904)
}
