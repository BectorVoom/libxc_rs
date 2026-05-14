//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 437/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk437<F: Float>(t1773: F, t46: F, t1634: F, t616: F, t1692: F, t615: F, t1701: F, t1703: F, t1706: F, t1708: F, t1712: F, t1718: F, t1723: F, t1728: F, t1733: F, t1736: F, t1756: F, t1760: F, t1768: F, t1770: F, t580: F, t590: F, t612: F) -> (F, F, F, F) {
    let t1774 = t1773 * t46;
    let t1775 = t616 * t1634;
    let t1776 = t1774 * t1775;
    let t1779 = t616 * t1692;
    let t1780 = t615 * t1779;
    let t1783 = t1701 + 7.0 / 72.0 * t1703 + t1706 * t1708 / 16.0 - t580 * t1712 / 48.0 + 0.42874018118069736972e-3 * t1718 * t1723 + 0.20007875121765877254e-2 * t1728 + 0.17149607247227894789e-2 * t1733 * t1736 - 0.21437009059034868486e-3 * t590 * t1756 - 0.21437009059034868486e-3 * t590 * t1760 + t1768 + 0.80031500487063509015e-2 * t1770 + 0.42874018118069736972e-2 * t612 * t1776 - 0.85748036236139473944e-3 * t612 * t1780;
    (t1774, t1776, t1780, t1783)
}
