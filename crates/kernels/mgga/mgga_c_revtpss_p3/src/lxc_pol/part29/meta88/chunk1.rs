//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 531/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk531<F: Float>(t1803: F, t467: F, t1264: F, t1715: F, t247: F, t1221: F, t1222: F, t1235: F, t1247: F, t1258: F, t1261: F, t1778: F, t1782: F, t1786: F, t1791: F, t1797: F, t464: F, t484: F) -> (F, F, F) {
    let t1804 = t467 * t1803;
    let t1807 = t1264 * t1715;
    let t1808 = t247 * t1807;
    let t1811 = -t1778 * t464 / F::cast_from(36.0_f64) + t1221 - t1222 * t1782 / F::cast_from(288.0_f64) + F::cast_from(0.21437009059034868486e-3_f64) * t1786 * t484 - F::cast_from(0.21437009059034868486e-3_f64) * t1235 * t1791 + F::cast_from(0.21437009059034868486e-3_f64) * t1247 * t1797 - F::cast_from(0.11433071498151929859e-2_f64) * t1804 * t484 + t1258 - F::cast_from(0.14291339372689912324e-3_f64) * t1261 * t1808;
    (t1804, t1808, t1811)
}
