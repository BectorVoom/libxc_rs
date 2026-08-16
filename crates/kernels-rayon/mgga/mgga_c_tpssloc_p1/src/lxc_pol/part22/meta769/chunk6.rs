//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2616/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2616(t15594: f64, t15737: f64, t18342: f64, t18594: f64, t19058: f64, t19101: f64, t5005: f64, t5024: f64, t53372: f64, t53399: f64, t6207: f64, t6227: f64, t6232: f64, t66406: f64, t66408: f64, t66410: f64, t66413: f64, t66437: f64) -> f64 {
    let t73019 = t5024 * t19101 / 288.0_f64 + t53372 * t6227 / 512.0_f64 - t53399 * t6232 / 1024.0_f64 - t15594 * t6207 / 1536.0_f64 - t5005 * t19101 / 1536.0_f64 + 5.0_f64 / 6912.0_f64 * t66406 - t66408 / 144.0_f64 + 19.0_f64 / 864.0_f64 * t66410 - t5005 * t18594 / 256.0_f64 - 5.0_f64 / 432.0_f64 * t5024 * t18342 + t15737 * t19058 / 512.0_f64 + t66413 / 384.0_f64 + t66437 / 256.0_f64;
    t73019
}
