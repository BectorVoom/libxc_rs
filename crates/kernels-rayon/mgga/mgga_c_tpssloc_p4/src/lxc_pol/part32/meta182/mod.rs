//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta182 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk892;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk893;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk894;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta182(t1155: f64, t3403: f64, t4882: f64, t1164: f64, t1171: f64, t1706: f64, t1420: f64, t972: f64, t1709: f64, t3431: f64, t1174: f64, t3439: f64, t60: f64, t461: f64, t4724: f64, t1409: f64, t3450: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4883, t4884, t4886, t4887, t4889) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk892(t1155, t3403, t4882, t1164, t1171, t1706, t1420, t972);
        let (t4896, t4897, t4899) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk893(t1709, t3431, t1174, t3439, t60);
        let (t4900, t4901, t4904) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk894(t461, t4899, t4724, t1409, t3450);
    (t4883, t4884, t4886, t4887, t4889, t4896, t4897, t4899, t4900, t4901, t4904)
}
