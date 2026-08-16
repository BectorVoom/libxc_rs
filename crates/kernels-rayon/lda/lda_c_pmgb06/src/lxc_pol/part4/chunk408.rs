//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 408/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk408(t1080: f64, t1464: f64, t1476: f64, t36: f64, t1414: f64, t506: f64, t1083: f64, t497: f64, t1473: f64, t1474: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1477 = t1464 * t1080;
    let t1478 = t1476 * t1477;
    let t1479 = t36 * t1478;
    let t1481 = t1414 * t1080;
    let t1482 = t506 * t1481;
    let t1483 = t36 * t1482;
    let t1485 = t497 * t1083;
    let t1486 = t506 * t1485;
    let t1487 = t36 * t1486;
    let t1489 = -t1473 - 0.0012594444444444445_f64 * t1474 + 0.0012594444444444445_f64 * t1479 - 0.003778333333333333_f64 * t1483 + 0.0018891666666666666_f64 * t1487;
    (t1477, t1478, t1479, t1481, t1482, t1483, t1485, t1486, t1487, t1489)
}
