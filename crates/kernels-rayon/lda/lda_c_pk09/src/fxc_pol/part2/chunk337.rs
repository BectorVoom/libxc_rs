//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 337/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk337(t1610: f64, t93: f64, t303: f64, t305: f64, t1303: f64, t1272: f64, t1217: f64, t1451: f64, t1562: f64, t1565: f64, t1568: f64, t1581: f64, t1594: f64, t1597: f64, t1601: f64, t1602: f64, t1604: f64, t1606: f64, t1608: f64, t1609: f64, t311: f64) -> (f64, f64, f64, f64, f64) {
    let t1611 = t93 * t1610;
    let t1614 = t303 * t305;
    let t1615 = t1303 * t1614;
    let t1618 = 0.03412591035063918_f64 * t1272;
    let t1619 = -t1562 * t311 / 6.0_f64 - t1565 * t311 / 6.0_f64 - t1568 * t311 / 6.0_f64 - t1581 * t311 / 6.0_f64 + t1594 * t311 / 6.0_f64 + t1597 * t311 / 6.0_f64 + 0.14975624337724558_f64 * t1217 - t1601 - t1602 + t1604 - t1606 + t1608 + t1609 * t1611 / 12.0_f64 - t1615 * t1451 / 6.0_f64 - t1618;
    (t1611, t1614, t1615, t1618, t1619)
}
