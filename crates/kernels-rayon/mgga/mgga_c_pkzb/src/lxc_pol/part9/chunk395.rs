//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 395/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk395(t501: f64, t546: f64, t496: f64, t513: f64, t1501: f64, t1510: f64, t1513: f64, t1520: f64, t1530: f64, t1534: f64, t1535: f64, t1536: f64, t1544: f64, t1547: f64, t1550: f64, t1553: f64, t568: f64) -> (f64, f64, f64, f64, f64) {
    let t1555 = 8.0_f64 * t501 * t546;
    let t1556 = t496 * t513;
    let t1557 = 8.0_f64 * t1556;
    let t1559 = 8.0_f64 * t496 * t546;
    let t1560 = 6.0_f64 * t1535 * t1536 * t568 - t1501 - t1510 - t1513 - t1520 + t1530 + t1534 + t1544 + t1547 - t1550 - t1553 - t1555 + t1557 + t1559;
    (t1555, t1556, t1557, t1559, t1560)
}
