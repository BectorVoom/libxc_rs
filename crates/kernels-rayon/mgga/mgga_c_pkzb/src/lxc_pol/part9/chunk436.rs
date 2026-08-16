//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 436/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk436(t1734: f64, t568: f64, t179: f64, t1501: f64, t1510: f64, t1513: f64, t1520: f64, t1555: f64, t1557: f64, t1559: f64, t1627: f64, t1630: f64, t1632: f64, t1663: f64) -> (f64, f64) {
    let t1735 = t1734 * t568;
    let t1736 = t179 * t1735;
    let t1739 = t1627 - t1501 - t1510 - t1513 + t1663 + t1630 - t1632 - t1555 + t1557 + t1559 - t1520;
    (t1736, t1739)
}
