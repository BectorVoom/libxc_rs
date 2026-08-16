//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta153 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk760;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk761;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta153<F: Float>(t4205: F, t708: F, t1462: F, t2427: F, t2373: F, t2377: F, t2408: F, t4097: F, t4099: F, t4100: F, t4103: F, t4198: F, t4201: F, t4204: F, t1474: F, t67: F, t758: F, t2431: F, t2532: F, t2653: F, t2417: F, t2423: F, t2426: F, t2486: F, t2518: F, t2530: F, t2537: F, t2538: F, t2665: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t4207, t4209, t4210) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk760::<F>(t4205, t708, t1462, t2427, t2373, t2377, t2408, t4097, t4099, t4100, t4103, t4198, t4201, t4204);
        let (t4211, t4212, t4213, t4214, t4215, t4216, t4217) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk761::<F>(t1474, t67, t758, t2431, t2532, t2653, t2417, t2423, t2426, t2486, t2518, t2530, t2537, t2538, t2665);
    (t4207, t4209, t4210, t4211, t4212, t4213, t4214, t4215, t4216, t4217)
}
