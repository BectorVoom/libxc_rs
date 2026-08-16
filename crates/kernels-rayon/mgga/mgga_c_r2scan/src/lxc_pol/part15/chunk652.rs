//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 652/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk652(t322: f64, t3413: f64, t3420: f64, t352: f64, t3644: f64, t3646: f64, t3674: f64, t3675: f64, t3678: f64, t855: f64, t3446: f64, t3453: f64, t970: f64) -> (f64, f64) {
    let t323 = t322 <= 0.0_f64;
    let t331 = t322 <= 0.25e1_f64;
    let t3685 = piecewise5(t323, t3644 + t3646, t331, t3674, -0.21e1_f64 * t3413 * t3675 - 0.105e1_f64 * t855 * t3678 * t352 - 0.1575e1_f64 * t3420 * t3675);
    let t3690 = t3446 * t3453 * t970;
    (t3685, t3690)
}
