//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1084/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1084<F: Float>(t1464: F, t1913: F, t10270: F, t10272: F, t10279: F, t10281: F, t10288: F, t10290: F, t10275: F, t10278: F, t10284: F, t10287: F, t10295: F) -> (F, F) {
    let t13256 = F::new(2.0) * t1913 * t1464;
    let t13261 = F::new(4.0) * t10270;
    let t13262 = F::new(12.0) * t10272;
    let t13263 = F::new(48.0) * t10279;
    let t13264 = F::new(80.0) * t10281;
    let t13265 = F::new(180.0) * t10288;
    let t13266 = F::new(252.0) * t10290;
    let t13267 = t13261 + t13262 - t10275 - t10278 + t13263 + t13264 - t10284 - t10287 + t13265 + t13266 - t10295;
    (t13256, t13267)
}
