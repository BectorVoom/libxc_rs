//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 375/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk375<F: Float>(t1221: F, t1222: F, t1235: F, t1247: F, t1258: F, t1261: F, t1778: F, t1782: F, t1786: F, t1791: F, t1797: F, t1804: F, t1808: F, t464: F, t484: F) -> F {
    let t1811 = -t1778 * t464 / F::new(36.0) + t1221 - t1222 * t1782 / F::new(288.0) + F::new(0.21437009059034868486e-3) * t1786 * t484 - F::new(0.21437009059034868486e-3) * t1235 * t1791 + F::new(0.21437009059034868486e-3) * t1247 * t1797 - F::new(0.11433071498151929859e-2) * t1804 * t484 + t1258 - F::new(0.14291339372689912324e-3) * t1261 * t1808;
    t1811
}
