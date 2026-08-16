//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 376/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk376(t1118: f64, t1670: f64, t1099: f64, t1122: f64, t1655: f64, t1131: f64, t1134: f64, t1662: f64, t1665: f64, t1668: f64, t1137: f64, t1141: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1671 = t1670 * t1118;
    let t1673 = 1.0_f64 * t1099 * t1671;
    let t1675 = -t1122 + 0.17123333333333333333e-1_f64 * t1655;
    let t1682 = 0.3529725e1_f64 * t1662 - t1131 + 0.516475e0_f64 * t1655 + 0.6311625e0_f64 * t1665 - t1134 + 0.104195e0_f64 * t1668;
    let t1683 = t1682 * t1137;
    let t1687 = -t1141 + 0.92708333333333333333e-2_f64 * t1655;
    (t1671, t1673, t1675, t1682, t1683, t1687)
}
