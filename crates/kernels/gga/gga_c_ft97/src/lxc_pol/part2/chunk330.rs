//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 330/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk330<F: Float>(t1557: F, t1736: F, t1559: F, t420: F, t419: F, t1570: F, t422: F, t1580: F, t423: F, t1718: F, t1722: F, t1726: F, t1731: F, t1733: F, t409: F, t1300: F, t1596: F, t1599: F, t1603: F, t1605: F, t1617: F, t1621: F, t1624: F, t1626: F, t1633: F, t1657: F, t1660: F, t1665: F, t1669: F, t1671: F, t1683: F, t1687: F, t1698: F, t1704: F, t1713: F, t372: F, t399: F, t403: F, t64: F, t79: F) -> (F, F, F, F, F, F, F, F) {
    let t1737 = t1736 * t1557;
    let t1738 = t1737 * t1559;
    let t1739 = t420 * t1738;
    let t1740 = t419 * t1739;
    let t1742 = t422 * t1570;
    let t1743 = t1742 * t1559;
    let t1744 = t420 * t1743;
    let t1745 = t419 * t1744;
    let t1747 = t423 * t1580;
    let t1748 = t420 * t1747;
    let t1749 = t419 * t1748;
    let t1751 = 0.18727458458024691358e0 * t1718 - 0.3404992446913580247e-1 * t1722 - 0.3404992446913580247e-1 * t1726 - t1731 + 0.42562405586419753086e-2 * t1733 + 0.85124811172839506173e-2 * t1740 - 0.12768721675925925926e-1 * t1745 + 0.6384360837962962963e-2 * t1749;
    let t1752 = t409 * t1751;
    let t1754 = 0.67598802253579164263e-4 * t1596 * t1599 - 0.46509801892875584e-1 * t1603 * t1605 - 0.13784064983740990796e-3 * t1617 * t1621 + 0.23254900946437792e-1 * t1624 * t1626 + 0.23254900946437792e-2 * t372 * t1633 - 0.11627450473218896e-1 * t372 * t1657 + 0.19365723406274399941e-3 * t372 * t1660 + 2.0 * t1665 + 0.2370952259137005195e-1 * t403 * t399 - 4.0 * t1669 * t1671 + 2.0 * t1687 + 0.14053536537767171586e-3 * t79 * t1698 - 0.11854761295685025975e-1 * t1300 * t1704 - 0.37540077436335915588e-1 * t79 * t1683 + 2.0 * t64 * t1713 - t64 * t1752;
    (t1738, t1740, t1743, t1745, t1747, t1749, t1751, t1754)
}
