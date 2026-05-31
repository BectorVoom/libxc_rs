//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 335/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk335<F: Float>(t1593: F, t306: F, t1215: F, t327: F, t1220: F, t1224: F, t1435: F, t319: F, t328: F, t307: F, t1387: F, t1506: F) -> (F, F, F, F, F, F, F, F) {
    let t1594 = t1593 * t306;
    let t1597 = t327 * t1215;
    let t1601 = F::cast_from(0.01233429741534199_f64) * t1220;
    let t1602 = F::cast_from(0.04991874779241519_f64) * t1224;
    let t1604 = t319 * t1435 / F::cast_from(18.0_f64);
    let t1606 = t328 * t1435 / F::cast_from(18.0_f64);
    let t1608 = t307 * t1435 / F::cast_from(18.0_f64);
    let t1609 = t1506 * t1387;
    (t1594, t1597, t1601, t1602, t1604, t1606, t1608, t1609)
}
