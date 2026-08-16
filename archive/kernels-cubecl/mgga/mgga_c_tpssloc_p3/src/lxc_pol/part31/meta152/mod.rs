//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta152 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk758;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk759;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta152<F: Float>(t2645: F, t2647: F, t4181: F, t157: F, t2658: F, t1409: F, t184: F, t607: F, t1474: F, t172: F, t763: F, t185: F, t3966: F, t707: F, t1471: F, t706: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t4191 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk758::<F>(t2645, t2647, t4181);
        let (t4194, t4195, t4196, t4198, t4199, t4200, t4201, t4202, t4204, t4205) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk759::<F>(t157, t2658, t1409, t184, t607, t1474, t172, t763, t185, t3966, t707, t1471, t706);
    (t4191, t4194, t4195, t4196, t4198, t4199, t4200, t4201, t4202, t4204, t4205)
}
