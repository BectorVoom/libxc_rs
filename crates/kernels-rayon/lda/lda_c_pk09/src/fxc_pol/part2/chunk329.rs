//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 329/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk329(t1478: f64, t1513: f64, t1292: f64, t1457: f64, t1460: f64, t1462: f64, t1464: f64, t1466: f64, t1474: f64, t1476: f64, t1481: f64, t1483: f64, t1489: f64, t1490: f64, t1495: f64, t1504: f64, t1507: f64, t1510: f64, t311: f64, t410: f64) -> (f64, f64) {
    let t1514 = t1513 * t1478;
    let t1516 = -t1457 + t1460 - t1462 - t1464 + t1466 + t1474 - 7.108175748183851_f64 * t1476 * t1478 + 7.108175748183851_f64 * t1481 * t1483 + t1489 + 2.427516195194328_f64 * t1490 * t1292 - t1495 * t1504 - 1.8805371096875316_f64 * t1507 * t410 - 3.7610742193750633_f64 * t1510 * t311 - 5.40024514194619_f64 * t1514;
    (t1514, t1516)
}
