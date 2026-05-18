//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 331/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk331<F: Float>(t1287: F, t1519: F, t1292: F, t332: F, t1468: F, t281: F, t10: F, t1472: F, t1435: F, t360: F, t348: F, t1214: F, t403: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1520 = t1519 * t1287;
    let t1521 = F::new(22.07984838129906) * t1520;
    let t1522 = t332 * t1292;
    let t1524 = t1468 * t281;
    let t1525 = t1524 * t10;
    let t1526 = t1525 * t1472;
    let t1527 = F::new(5.40024514194619) * t1526;
    let t1529 = F::new(6.496391258193384) * t360 * t1435;
    let t1531 = F::new(1.2536914064583544) * t348 * t1435;
    let t1532 = t403 * t1214;
    (t1520, t1521, t1522, t1524, t1525, t1526, t1527, t1529, t1531, t1532)
}
