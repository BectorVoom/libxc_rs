//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 643/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk643(t1403: f64, t5308: f64, t1336: f64, t1426: f64, t1625: f64, t1424: f64, t1422: f64, t387: f64, t5141: f64, t1421: f64, t1397: f64, t1413: f64) -> (f64, f64, f64, f64) {
    let t5547 = t1403 * t5308;
    let t5549 = t1426 * t1336;
    let t5550 = t5549 * t1625;
    let t5554 = t1424 * t1424;
    let t5555 = 1.0_f64 / t5554;
    let t5558 = t387 * t1422;
    let t5561 = 1.0_f64 / t5141;
    let t5562 = t1421 * t5561;
    let t5564 = t1397 * t5562 - 2.0_f64 * t1413 * t5558;
    (t5547, t5550, t5555, t5564)
}
