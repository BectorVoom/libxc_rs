//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 441/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk441(t1773: f64, t46: f64, t1634: f64, t616: f64, t1692: f64, t615: f64, t1701: f64, t1703: f64, t1706: f64, t1708: f64, t1712: f64, t1718: f64, t1723: f64, t1728: f64, t1733: f64, t1736: f64, t1756: f64, t1760: f64, t1768: f64, t1770: f64, t580: f64, t590: f64, t612: f64) -> (f64, f64, f64, f64) {
    let t1774 = t1773 * t46;
    let t1775 = t616 * t1634;
    let t1776 = t1774 * t1775;
    let t1779 = t616 * t1692;
    let t1780 = t615 * t1779;
    let t1783 = t1701 + 7.0_f64 / 72.0_f64 * t1703 + t1706 * t1708 / 16.0_f64 - t580 * t1712 / 48.0_f64 + 0.42874018118069736972e-3_f64 * t1718 * t1723 + 0.20007875121765877254e-2_f64 * t1728 + 0.17149607247227894789e-2_f64 * t1733 * t1736 - 0.21437009059034868486e-3_f64 * t590 * t1756 - 0.21437009059034868486e-3_f64 * t590 * t1760 + t1768 + 0.80031500487063509015e-2_f64 * t1770 + 0.42874018118069736972e-2_f64 * t612 * t1776 - 0.85748036236139473944e-3_f64 * t612 * t1780;
    (t1774, t1776, t1780, t1783)
}
