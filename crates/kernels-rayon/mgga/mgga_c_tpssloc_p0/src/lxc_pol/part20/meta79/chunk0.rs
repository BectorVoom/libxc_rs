//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 565/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk565(t265: f64, t394: f64, t1052: f64, t1604: f64, t1626: f64, t1635: f64, t388: f64, t1070: f64, t1534: f64, t1545: f64, t1559: f64, t1585: f64, t1587: f64, t1591: f64, t193: f64, t336: f64) -> (f64, f64) {
    let t395 = t265 < t394;
    let t1637 = -t1052 * t1635 + t1604 * t388 + t1626 * t388;
    let t1642 = piecewise3(t395, t1070 * t1637 * t193 * t336 - t1545 + t1559 + t1585 + t1587 - t1591, t1534);
    (t1637, t1642)
}
