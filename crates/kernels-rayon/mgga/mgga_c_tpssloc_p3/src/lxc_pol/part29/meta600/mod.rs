//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta600 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2033;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2034;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta600(t23109: f64, t23110: f64, t232: f64, t236: f64, t2678: f64, t23102: f64, t80782: f64, t23113: f64, t23093: f64, t281: f64, t23046: f64, t812: f64, t835: f64, t2635: f64, t23041: f64, t2681: f64, t22690: f64, t23122: f64, t2553: f64, t841: f64, t22813: f64, t6589: f64, t23124: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81874, t81876, t81877, t81882, t81883, t81886) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2033(t23109, t23110, t232, t236, t2678, t23102, t80782, t23113, t23093, t281, t23046, t812, t835);
        let (t81887, t81889, t81899, t81902, t81903) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2034(t2635, t81886, t23041, t2681, t22690, t23122, t2553, t841, t22813, t6589, t80782, t23124);
    (t81874, t81876, t81877, t81882, t81883, t81887, t81889, t81899, t81902, t81903)
}
