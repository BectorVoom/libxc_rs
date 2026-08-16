//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta317 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1390;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta317(t407: f64, t11135: f64, t410: f64, t417: f64, t1097: f64, t3311: f64, t409: f64, t3314: f64, t422: f64, t1146: f64, t3399: f64, t3402: f64, t448: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t11243, t11247, t11265, t11275, t11277, t11282, t11285) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1390(t407, t11135, t410, t417, t1097, t3311, t409, t3314, t422, t1146, t3399, t3402, t448);
    (t11243, t11247, t11265, t11275, t11277, t11282, t11285)
}
