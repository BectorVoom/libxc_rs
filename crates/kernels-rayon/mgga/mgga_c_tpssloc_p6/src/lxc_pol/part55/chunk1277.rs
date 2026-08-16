//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1277/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1277(t32448: f64, t4993: f64, t10401: f64, t117963: f64, t117969: f64, t117973: f64, t119243: f64, t1218: f64, t1232: f64, t125398: f64, t125402: f64, t125407: f64, t125410: f64, t125413: f64, t1737: f64, t32439: f64, t32441: f64, t3500: f64, t4983: f64, t5014: f64) -> f64 {
    let t125420 = t32448 * t4993;
    let t125424 = -t3500 * t32439 * t10401 * t119243 * t4983 / 1536.0_f64 + t125398 / 2304.0_f64 - t125402 * t1218 / 288.0_f64 + t125407 * t1232 / 432.0_f64 + t125410 * t1218 / 1536.0_f64 - t125413 * t1232 / 2304.0_f64 + t117973 * t1737 / 1536.0_f64 + t32441 * t5014 / 1536.0_f64 - t125420 / 3456.0_f64 - t117963 / 3456.0_f64 + t117969 / 2304.0_f64;
    t125424
}
