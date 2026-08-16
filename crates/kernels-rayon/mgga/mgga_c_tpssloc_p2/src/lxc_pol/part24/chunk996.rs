//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 996/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk996(t11228: f64, t11268: f64, t1118: f64, t1099: f64, t1097: f64, t3311: f64, t409: f64, t3314: f64, t422: f64, t11191: f64, t1146: f64, t3399: f64) -> (f64, f64, f64) {
    let t11269 = t11228 + t11268;
    let t11270 = t11269 * t1118;
    let t11272 = 1.0_f64 * t1099 * t11270;
    let t11274 = 1.0_f64 / t3311 / t1097;
    let t11275 = t409 * t11274;
    let t11277 = 1.0_f64 / t3314 / t422;
    let t11278 = t11191 * t11277;
    let t11280 = 0.51726012919273400301e3_f64 * t11275 * t11278;
    let t11282 = 1.0_f64 / t3399 / t1146;
    (t11272, t11280, t11282)
}
