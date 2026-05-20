//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2206/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2206<F: Float>(t29506: F, t7316: F, t30112: F, t7235: F, t27833: F, t7937: F, t28189: F, t7898: F, t7239: F, t2014: F, t30111: F, t7315: F) -> (F, F, F, F, F, F) {
    let t109158 = t29506 * t7316;
    let t109159 = t7235 * t30112;
    let t109162 = F::new(2.0) * t27833 * t7937;
    let t109164 = F::new(2.0) * t7898 * t28189;
    let t109167 = F::new(3.0) * t29506 * t7239;
    let t109169 = t2014 * t30111 * t7315;
    (t109158, t109159, t109162, t109164, t109167, t109169)
}
