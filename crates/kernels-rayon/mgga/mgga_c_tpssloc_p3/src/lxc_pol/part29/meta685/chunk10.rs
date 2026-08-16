//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2348/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2348(t111: f64, t27370: f64, t12813: f64, t1458: f64, t2363: f64, t24932: f64, t27863: f64, t27888: f64, t4072: f64, t671: f64, t7266: f64, t85428: f64, t90355: f64, t90361: f64, t90363: f64, t90365: f64, t90367: f64, t90369: f64, t94248: f64, t96222: f64) -> (f64, f64) {
    let t96238 = t27370 * t111;
    let t96269 = 2.0_f64 * t12813 * t7266 + 2.0_f64 * t1458 * t85428 + 2.0_f64 * t1458 * t94248 + 4.0_f64 * t1458 * t96222 + 2.0_f64 * t2363 * t27863 + 4.0_f64 * t24932 * t4072 + 4.0_f64 * t27888 * t4072 + 4.0_f64 * t671 * t96238 + t90355 + t90361 + t90363 + t90365 + t90367 + t90369;
    (t96238, t96269)
}
