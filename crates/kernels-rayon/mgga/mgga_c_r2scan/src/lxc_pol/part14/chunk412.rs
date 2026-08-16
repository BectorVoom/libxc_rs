//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 412/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk412(t1654: f64, t170: f64, t188: f64, t631: f64, t189: f64, t621: f64, t390: f64, t649: f64, t652: f64, t124: f64, t4: f64, t615: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1655 = t1654 * t170;
    let t1658 = t631 * t188;
    let t1659 = t189 * t621;
    let t1660 = t1658 * t1659;
    let t1662 = 0.71233333333333333332e-1_f64 * t390 * t1660;
    let t1663 = t649 * t188;
    let t1664 = t652 * t621;
    let t1665 = t1663 * t1664;
    let t1667 = 0.57278650314509912396e0_f64 * t390 * t1665;
    let t1668 = t4 * t124;
    let t1669 = t615 * t1668;
    (t1655, t1658, t1659, t1660, t1662, t1663, t1664, t1665, t1667, t1668, t1669)
}
