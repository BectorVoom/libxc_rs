//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 329/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk329<F: Float>(t1478: F, t1513: F, t1292: F, t1457: F, t1460: F, t1462: F, t1464: F, t1466: F, t1474: F, t1476: F, t1481: F, t1483: F, t1489: F, t1490: F, t1495: F, t1504: F, t1507: F, t1510: F, t311: F, t410: F) -> (F, F) {
    let t1514 = t1513 * t1478;
    let t1516 = -t1457 + t1460 - t1462 - t1464 + t1466 + t1474 - F::cast_from(7.108175748183851_f64) * t1476 * t1478 + F::cast_from(7.108175748183851_f64) * t1481 * t1483 + t1489 + F::cast_from(2.427516195194328_f64) * t1490 * t1292 - t1495 * t1504 - F::cast_from(1.8805371096875316_f64) * t1507 * t410 - F::cast_from(3.7610742193750633_f64) * t1510 * t311 - F::cast_from(5.40024514194619_f64) * t1514;
    (t1514, t1516)
}
