//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 298/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk298(t1378: f64, t85: f64, t483: f64, t75: f64, t288: f64, t224: f64, t484: f64, t229: f64, t87: f64, t40: f64, t276: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1379 = t1378 * t85;
    let t1380 = 0.19751673498613801407e-1_f64 * t1379;
    let t1381 = t483 * t75;
    let t1382 = t1381 * t288;
    let t1383 = 0.5848223622634646207e0_f64 * t1382;
    let t1384 = t224 * t484;
    let t1385 = 4.0_f64 * t1384;
    let t1386 = t229 * t484;
    let t1387 = 4.0_f64 * t1386;
    let t1388 = t1378 * t87;
    let t1389 = t40 * t1388;
    let t1390 = t483 * t276;
    let t1391 = t40 * t1390;
    (t1380, t1381, t1382, t1383, t1384, t1385, t1386, t1387, t1388, t1389, t1390, t1391)
}
