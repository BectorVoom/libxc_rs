//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 335/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk335(t1593: f64, t306: f64, t1215: f64, t327: f64, t1220: f64, t1224: f64, t1435: f64, t319: f64, t328: f64, t307: f64, t1387: f64, t1506: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1594 = t1593 * t306;
    let t1597 = t327 * t1215;
    let t1601 = 0.01233429741534199_f64 * t1220;
    let t1602 = 0.04991874779241519_f64 * t1224;
    let t1604 = t319 * t1435 / 18.0_f64;
    let t1606 = t328 * t1435 / 18.0_f64;
    let t1608 = t307 * t1435 / 18.0_f64;
    let t1609 = t1506 * t1387;
    (t1594, t1597, t1601, t1602, t1604, t1606, t1608, t1609)
}
