//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1298/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1298<F: Float>(t10272: F, t10279: F, t10281: F, t10288: F, t10290: F, t10275: F, t10278: F, t10284: F, t10287: F, t10295: F, t13261: F, t4171: F, t602: F) -> (F, F) {
    let t13262 = F::new(12.0) * t10272;
    let t13263 = F::new(48.0) * t10279;
    let t13264 = F::new(80.0) * t10281;
    let t13265 = F::new(180.0) * t10288;
    let t13266 = F::new(252.0) * t10290;
    let t13267 = t13261 + t13262 - t10275 - t10278 + t13263 + t13264 - t10284 - t10287 + t13265 + t13266 - t10295;
    let t13269 = t4171 * t602;
    (t13267, t13269)
}
