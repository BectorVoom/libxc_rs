//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta254 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1374;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1375;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1376;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta254(t11135: f64, t10292: f64, t281: f64, t415: f64, t1114: f64, t2403: f64, t241: f64, t3439: f64, t407: f64, t410: f64, t417: f64, t1097: f64, t3311: f64, t409: f64, t3314: f64, t422: f64, t1146: f64, t3399: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11195, t11203, t11204, t11211, t11219, t11243, t11247, t11265, t11274) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1374(t11135, t10292, t281, t415, t1114, t2403, t241, t3439, t407, t410, t417, t1097, t3311);
        let t11275 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1375(t11274, t409);
        let (t11277, t11282) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1376(t3314, t422, t1146, t3399);
    (t11195, t11203, t11204, t11211, t11219, t11243, t11247, t11265, t11274, t11275, t11277, t11282)
}
