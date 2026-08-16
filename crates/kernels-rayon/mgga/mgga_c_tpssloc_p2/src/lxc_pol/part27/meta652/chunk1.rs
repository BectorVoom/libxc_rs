//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2275/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2275(t1873: f64, t90375: f64, t22479: f64, t4028: f64, t1458: f64, t2363: f64, t24999: f64, t83935: f64, t90351: f64, t90352: f64, t90355: f64, t90361: f64, t90363: f64, t90365: f64, t90367: f64, t90369: f64, t90372: f64, t90374: f64) -> f64 {
    let t90377 = 2.0_f64 * t90375 * t1873;
    let t90379 = 2.0_f64 * t4028 * t22479;
    let t90380 = 2.0_f64 * t1458 * t83935 + 2.0_f64 * t2363 * t24999 + t90351 + 2.0_f64 * t90352 + t90355 + t90361 + t90363 + t90365 + t90367 + t90369 + t90372 + t90374 + t90377 + t90379;
    t90380
}
