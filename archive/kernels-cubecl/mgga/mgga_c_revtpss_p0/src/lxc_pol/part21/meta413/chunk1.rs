//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1886/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1886<F: Float>(t10270: F, t10272: F, t10279: F, t10281: F, t10288: F, t10290: F, t10275: F, t10278: F, t10284: F, t10287: F, t10295: F, t4171: F, t602: F) -> (F, F) {
    let t13261 = F::cast_from(4.0_f64) * t10270;
    let t13262 = F::cast_from(12.0_f64) * t10272;
    let t13263 = F::cast_from(48.0_f64) * t10279;
    let t13264 = F::cast_from(80.0_f64) * t10281;
    let t13265 = F::cast_from(180.0_f64) * t10288;
    let t13266 = F::cast_from(252.0_f64) * t10290;
    let t13267 = t13261 + t13262 - t10275 - t10278 + t13263 + t13264 - t10284 - t10287 + t13265 + t13266 - t10295;
    let t13269 = t4171 * t602;
    (t13267, t13269)
}
