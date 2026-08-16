//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 482/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk482(t1388: f64, t1462: f64, t1464: f64, t1466: f64, t1474: f64, t1476: f64, t1481: f64, t1489: f64, t1490: f64, t1521: f64, t1527: f64, t1529: f64, t1531: f64, t2513: f64, t2517: f64, t2521: f64, t2675: f64, t2690: f64, t311: f64) -> f64 {
    let t2693 = -t1462 - t1464 + t1466 - 1.7770439370459628_f64 * t1388 * t2675 - 7.108175748183851_f64 * t1476 * t2517 + 7.108175748183851_f64 * t1481 * t2521 + 2.427516195194328_f64 * t1490 * t2513 - 2.427516195194328_f64 * t2690 * t311 + t1474 + t1489 + t1521 + t1527 - t1529 + t1531;
    t2693
}
