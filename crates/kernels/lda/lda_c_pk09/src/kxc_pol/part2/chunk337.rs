//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 337/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk337<F: Float>(t1610: F, t93: F, t303: F, t305: F, t1303: F, t1272: F, t1217: F, t1451: F, t1562: F, t1565: F, t1568: F, t1581: F, t1594: F, t1597: F, t1601: F, t1602: F, t1604: F, t1606: F, t1608: F, t1609: F, t311: F) -> (F, F, F, F, F) {
    let t1611 = t93 * t1610;
    let t1614 = t303 * t305;
    let t1615 = t1303 * t1614;
    let t1618 = F::cast_from(0.03412591035063918_f64) * t1272;
    let t1619 = -t1562 * t311 / F::cast_from(6.0_f64) - t1565 * t311 / F::cast_from(6.0_f64) - t1568 * t311 / F::cast_from(6.0_f64) - t1581 * t311 / F::cast_from(6.0_f64) + t1594 * t311 / F::cast_from(6.0_f64) + t1597 * t311 / F::cast_from(6.0_f64) + F::cast_from(0.14975624337724558_f64) * t1217 - t1601 - t1602 + t1604 - t1606 + t1608 + t1609 * t1611 / F::cast_from(12.0_f64) - t1615 * t1451 / F::cast_from(6.0_f64) - t1618;
    (t1611, t1614, t1615, t1618, t1619)
}
