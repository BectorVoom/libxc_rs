//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2038/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2038(t101091: f64, t101134: f64, t102105: f64, t102320: f64, t102366: f64, t102988: f64, t103029: f64, t103070: f64, t100930: f64, t1458: f64, t16521: f64, t16524: f64, t19534: f64, t20162: f64, t20173: f64, t20181: f64, t2039: f64, t24465: f64, t27170: f64, t27254: f64, t27281: f64, t28893: f64, t28951: f64, t29422: f64, t29425: f64, t3941: f64, t4072: f64, t5456: f64, t5493: f64, t55353: f64, t577: f64, t66958: f64, t671: f64, t7056: f64, t7801: f64, t7956: f64, t84033: f64) -> (f64, f64) {
    let t103073 = t101091 + t101134 + t102105 + t102320 + t102366 + t102988 + t103029 + t103070;
    let t103088 = 54.0_f64 * t20173 * t29422 + 54.0_f64 * t3941 * t27170 * t1458 + 54.0_f64 * t3941 * t7801 * t4072 + 54.0_f64 * t16524 * t27281 + 27.0_f64 * t20173 * t29425 + 27.0_f64 * t3941 * t28951 * t671 + 27.0_f64 * t100930 * t2039 + 27.0_f64 * t84033 * t5456 + 27.0_f64 * t28893 * t7056 + 27.0_f64 * t16521 * t7801 + 0.135e2_f64 * t20162 * t7056 + 27.0_f64 * t27254 * t4072 + 0.45e1_f64 * t103073 * t577 + 54.0_f64 * t55353 * t7956 + 27.0_f64 * t3941 * t7056 * t5493 + 27.0_f64 * t3941 * t2039 * t19534 + 27.0_f64 * t24465 * t20181 + 0.135e2_f64 * t66958 * t2039;
    (t103073, t103088)
}
