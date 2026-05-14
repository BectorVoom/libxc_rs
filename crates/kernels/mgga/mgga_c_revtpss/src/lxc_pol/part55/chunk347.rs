//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 347/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk347<F: Float>(t1794: F, t482: F, t1250: F, t1042: F, t476: F, t51: F, t52: F, t475: F, t467: F, t1264: F, t1715: F, t247: F, t1221: F, t1222: F, t1235: F, t1247: F, t1258: F, t1261: F, t1778: F, t1782: F, t1786: F, t1791: F, t464: F, t484: F) -> (F, F, F, F, F, F, F) {
    let t1795 = t482 * t1794;
    let t1796 = t1795 * t1250;
    let t1797 = t1042 * t1796;
    let t1800 = t476 * t51;
    let t1802 = 1.0 / t52 / t1800;
    let t1803 = t475 * t1802;
    let t1804 = t467 * t1803;
    let t1807 = t1264 * t1715;
    let t1808 = t247 * t1807;
    let t1811 = -t1778 * t464 / 36.0 + t1221 - t1222 * t1782 / 288.0 + 0.21437009059034868486e-3 * t1786 * t484 - 0.21437009059034868486e-3 * t1235 * t1791 + 0.21437009059034868486e-3 * t1247 * t1797 - 0.11433071498151929859e-2 * t1804 * t484 + t1258 - 0.14291339372689912324e-3 * t1261 * t1808;
    (t1796, t1797, t1802, t1803, t1804, t1808, t1811)
}
