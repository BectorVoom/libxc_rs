//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta315 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1343;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta315<F: Float>(t154: F, t3584: F, t3241: F, t636: F, t52: F, t1094: F, t3312: F, t3311: F, t419: F, t409: F, t11135: F, t10292: F, t281: F, t415: F) -> (F, F, F, F, F, F, F) {
        let (t11145, t11147, t11153, t11185, t11190, t11195, t11203) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1343::<F>(t154, t3584, t3241, t636, t52, t1094, t3312, t3311, t419, t409, t11135, t10292, t281, t415);
    (t11145, t11147, t11153, t11185, t11190, t11195, t11203)
}
