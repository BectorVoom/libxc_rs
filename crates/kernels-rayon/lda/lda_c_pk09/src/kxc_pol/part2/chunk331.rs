//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 331/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk331(t1287: f64, t1519: f64, t1292: f64, t332: f64, t1468: f64, t281: f64, t10: f64, t1472: f64, t1435: f64, t360: f64, t348: f64, t1214: f64, t403: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1520 = t1519 * t1287;
    let t1521 = 22.07984838129906_f64 * t1520;
    let t1522 = t332 * t1292;
    let t1524 = t1468 * t281;
    let t1525 = t1524 * t10;
    let t1526 = t1525 * t1472;
    let t1527 = 5.40024514194619_f64 * t1526;
    let t1529 = 6.496391258193384_f64 * t360 * t1435;
    let t1531 = 1.2536914064583544_f64 * t348 * t1435;
    let t1532 = t403 * t1214;
    (t1520, t1521, t1522, t1524, t1525, t1526, t1527, t1529, t1531, t1532)
}
