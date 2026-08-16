//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta263 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1100;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1101;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1102;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1103;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1104;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1105;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta263(t3: f64, t7222: f64, t112: f64, t2098: f64, t2039: f64, t671: f64, t1401: f64, t3938: f64, t3941: f64, t577: f64, t7056: f64, t1184: f64, t460: f64, t33: f64, t3953: f64, t1437: f64, t79: f64, t72: f64, t1410: f64, t605: f64, t1433: f64, t71: f64, t1458: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7223, t7230) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1100(t3, t7222, t112, t2098);
        let (t7235, t7240, t7319, t7428) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1101(t2039, t671, t1401, t3938, t3941, t577, t7056, t7222, t7230, t1184, t460, t33, t3953);
        let (t7431, t7432) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1102(t1437, t79, t72);
        let t7435 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1103(t1410, t605);
        let t7445 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1104(t1433, t71);
        let t7458 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1105(t1458, t89);
    (t7223, t7230, t7235, t7240, t7319, t7428, t7431, t7432, t7435, t7445, t7458)
}
