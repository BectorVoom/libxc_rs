//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 440/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk440<F: Float>(t1730: F, t619: F, t1406: F, t220: F, t186: F, t616: F, t633: F, t663: F, t582: F, t611: F, t185: F, t1687: F, t1689: F, t1694: F, t1700: F, t1704: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t1732 = 8.0 / 15.0 * t1730 * t619;
    let t1733 = -t1406;
    let t1734 = t220 * t1733;
    let t1735 = t186 * t1734;
    let t1737 = 4.0 / 15.0 * t616 * t1735;
    let t1739 = 4.0 / 15.0 * t633 * t663;
    let t1740 = t582 * t611;
    let t1741 = t185 * t1740;
    let t1742 = 8.0 / 45.0 * t1741;
    let t1743 = 0.25188888888888888889e-2 * t1687;
    let t1748 = -t1743 - 0.12594444444444444445e-2 * t1689 + 0.12594444444444444445e-2 * t1694 - 0.37783333333333333334e-2 * t1700 + 0.18891666666666666667e-2 * t1704;
    (t1732, t1733, t1734, t1735, t1737, t1739, t1740, t1741, t1742, t1743, t1748)
}
