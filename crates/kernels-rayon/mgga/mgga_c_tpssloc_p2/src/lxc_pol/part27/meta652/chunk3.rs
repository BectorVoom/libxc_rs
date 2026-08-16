//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2277/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2277(t12739: f64, t7467: f64, t26135: f64, t5113: f64, t12813: f64, t1458: f64, t22461: f64, t26103: f64, t4072: f64, t6517: f64, t671: f64, t90041: f64, t90044: f64, t90383: f64, t90385: f64, t90387: f64, t90399: f64, t90400: f64, t90404: f64, t90406: f64) -> f64 {
    let t90408 = 2.0_f64 * t12739 * t7467;
    let t90410 = 4.0_f64 * t5113 * t26135;
    let t90411 = 2.0_f64 * t12813 * t6517 + 4.0_f64 * t1458 * t90041 + 2.0_f64 * t1458 * t90044 + 4.0_f64 * t22461 * t4072 + 4.0_f64 * t26103 * t4072 + 4.0_f64 * t671 * t90400 + t90383 + t90385 + t90387 + t90399 + t90404 + t90406 + t90408 + t90410;
    t90411
}
