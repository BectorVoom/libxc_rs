//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta317 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1345;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta317(t3314: f64, t422: f64, t1146: f64, t3399: f64, t3402: f64, t448: f64, t445: f64, t1143: f64, t3375: f64, t1124: f64, t3331: f64, t440: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t11277, t11282, t11285, t11292, t11297, t11303, t11310) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1345(t3314, t422, t1146, t3399, t3402, t448, t445, t1143, t3375, t1124, t3331, t440);
    (t11277, t11282, t11285, t11292, t11297, t11303, t11310)
}
