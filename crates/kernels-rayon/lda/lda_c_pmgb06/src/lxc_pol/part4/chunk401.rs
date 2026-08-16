//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 401/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk401(t1455: f64, t493: f64, t1412: f64, t1419: f64, t1422: f64, t1425: f64, t1429: f64, t1433: f64, t1443: f64, t1446: f64, t1449: f64, t1453: f64) -> (f64, f64) {
    let t1457 = t493 * t1455 / 45.0_f64;
    let t1458 = t1412 - t1419 + t1422 + t1425 + t1429 + t1433 + t1443 + t1446 + t1449 + t1453 + t1457;
    (t1457, t1458)
}
