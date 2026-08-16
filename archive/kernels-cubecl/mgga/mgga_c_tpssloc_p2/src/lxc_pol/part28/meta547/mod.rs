//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta547 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1815;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1816;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta547<F: Float>(t214: F, t2710: F, t23258: F, t6547: F, t794: F, t852: F, t6562: F, t6572: F, t23219: F, t23265: F, t23030: F, t23208: F, t23168: F, t23223: F, t1882: F, t81686: F, t9537: F, t213: F, t225: F, t23164: F, t23204: F, t23222: F, t23238: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t82124, t82131, t82133, t82135, t82143, t82145, t82147) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1815::<F>(t214, t2710, t23258, t6547, t794, t852, t6562, t6572, t23219, t23265, t23030, t23208);
        let (t82150, t82153, t82159, t82172, t82174) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1816::<F>(t23168, t23223, t1882, t81686, t9537, t213, t225, t852, t23164, t23204, t23222, t23238);
    (t82124, t82131, t82133, t82135, t82143, t82145, t82147, t82150, t82153, t82159, t82172, t82174)
}
