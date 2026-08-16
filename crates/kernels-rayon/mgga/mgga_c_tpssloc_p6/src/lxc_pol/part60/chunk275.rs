//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 275/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk275(t1687: f64, t449: f64, t1150: f64, t1153: f64, t1655: f64, t1662: f64, t1665: f64, t1668: f64, t1156: f64, t1129: f64, t1148: f64, t1659: f64, t1673: f64, t1675: f64, t1683: f64, t300: f64, t436: f64) -> (f64, f64, f64, f64) {
    let t1688 = t1687 * t449;
    let t1694 = 0.258925e1_f64 * t1662 - t1150 + 0.301925e0_f64 * t1655 + 0.16504875e0_f64 * t1665 - t1153 + 0.82785e-1_f64 * t1668;
    let t1695 = t1694 * t1156;
    let t1699 = t300 * (-0.310907e-1_f64 * t1675 * t436 + 1.0_f64 * t1129 * t1683 + t1659 - t1673 - 0.19751673498613801407e-1_f64 * t1688 + 0.5848223622634646207e0_f64 * t1148 * t1695);
    (t1688, t1694, t1695, t1699)
}
