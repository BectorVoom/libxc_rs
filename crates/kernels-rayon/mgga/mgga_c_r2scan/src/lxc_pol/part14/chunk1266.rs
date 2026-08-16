//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1266/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1266(t12060: f64, t37271: f64, t12206: f64, t37282: f64, t38251: f64, t38259: f64, t38261: f64, t39109: f64, t39113: f64, t39114: f64, t39115: f64, t40642: f64, t42277: f64, t42281: f64, t42284: f64, t42287: f64, t42290: f64) -> (f64, f64, f64) {
    let t42292 = 5.0_f64 / 8.0_f64 * t37271 * t12060;
    let t42294 = 3.0_f64 / 2.0_f64 * t37282 * t12206;
    let t42298 = t42277 - t39109 - 0.32326021979378162576e-5_f64 * t38251 - t42281 + t42284 - t42287 + t42290 - t42292 + t42294 - 0.60975299583150056624e-3_f64 * t38259 + 0.60975299583150056624e-3_f64 * t38261 - t39113 - t39114 - t39115 + 0.60975299583150056624e-3_f64 * t40642;
    (t42292, t42294, t42298)
}
