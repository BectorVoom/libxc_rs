//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1689/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1689<F: Float>(t10275: F, t10278: F, t10284: F, t10287: F, t10295: F, t13261: F, t13262: F, t13263: F, t13264: F, t13265: F, t13266: F, t5812: F, t602: F) -> (F, F) {
    let t21661 = t13261 - t13262 - t10275 + t10278 + t13263 - t13264 - t10284 + t10287 + t13265 - t13266 - t10295;
    let t21663 = t5812 * t602;
    (t21661, t21663)
}
