//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 382/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk382(t1395: f64, t477: f64, t137: f64, t132: f64, t1166: f64, t183: f64, t398: f64, t539: f64, t188: f64, t1368: f64, t1370: f64, t1374: f64, t1379: f64, t1384: f64, t1389: f64, t1391: f64, t1394: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1396 = t1395 * t477;
    let t1397 = t137 * t1396;
    let t1399 = t132 * t1397 / 15.0_f64;
    let t1400 = t1166 * t183;
    let t1403 = t398 * t539;
    let t1404 = t1403 * t188;
    let t1406 = t1368 + 0.21642082724729686_f64 * t1370 + t1374 + t1379 - t1384 - t1389 - t1391 - t1394 - t1399 + 4.0_f64 / 3.0_f64 * t1400 * t188 + 8.0_f64 / 3.0_f64 * t1404;
    (t1396, t1397, t1399, t1400, t1403, t1404, t1406)
}
