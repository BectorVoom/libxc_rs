//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta600 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2033;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2034;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta600<F: Float>(t23109: F, t23110: F, t232: F, t236: F, t2678: F, t23102: F, t80782: F, t23113: F, t23093: F, t281: F, t23046: F, t812: F, t835: F, t2635: F, t23041: F, t2681: F, t22690: F, t23122: F, t2553: F, t841: F, t22813: F, t6589: F, t23124: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t81874, t81876, t81877, t81882, t81883, t81886) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2033::<F>(t23109, t23110, t232, t236, t2678, t23102, t80782, t23113, t23093, t281, t23046, t812, t835);
        let (t81887, t81889, t81899, t81902, t81903) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2034::<F>(t2635, t81886, t23041, t2681, t22690, t23122, t2553, t841, t22813, t6589, t80782, t23124);
    (t81874, t81876, t81877, t81882, t81883, t81887, t81889, t81899, t81902, t81903)
}
