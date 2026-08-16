//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 365/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk365(t1603: f64, t381: f64, t1409: f64, t998: f64, t974: f64, t225: f64, t68: f64, t369: f64, t1545: f64, t1559: f64, t1585: f64, t1587: f64, t1591: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1604 = t1603 * t381;
    let t1606 = t998 * t1409;
    let t1607 = t974 * t1606;
    let t1610 = t1603 * t225;
    let t1611 = t1610 * t68;
    let t1612 = t1611 * t369;
    let t1615 = -t1545 + t1559 + t1585 + t1587 - t1591;
    (t1604, t1606, t1607, t1610, t1611, t1612, t1615)
}
