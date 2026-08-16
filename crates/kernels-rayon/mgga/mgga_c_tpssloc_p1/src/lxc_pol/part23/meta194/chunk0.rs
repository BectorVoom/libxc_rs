//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 830/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk830(t1097: f64, t3311: f64, t409: f64, t3314: f64, t422: f64, t1146: f64, t3399: f64) -> (f64, f64, f64, f64) {
    let t11274 = 1.0_f64 / t3311 / t1097;
    let t11275 = t409 * t11274;
    let t11277 = 1.0_f64 / t3314 / t422;
    let t11282 = 1.0_f64 / t3399 / t1146;
    (t11274, t11275, t11277, t11282)
}
