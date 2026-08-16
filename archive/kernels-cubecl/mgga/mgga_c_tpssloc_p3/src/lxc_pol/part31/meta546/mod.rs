//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta546 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1771;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta546<F: Float>(t23145: F, t2617: F, t23102: F, t80782: F, t23113: F, t23093: F, t281: F, t23046: F, t812: F, t835: F, t22813: F, t6589: F) -> (F, F, F, F, F, F, F) {
        let (t81865, t81876, t81877, t81882, t81883, t81886, t81902) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1771::<F>(t23145, t2617, t23102, t80782, t23113, t23093, t281, t23046, t812, t835, t22813, t6589);
    (t81865, t81876, t81877, t81882, t81883, t81886, t81902)
}
