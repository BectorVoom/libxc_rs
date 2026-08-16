//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 623/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk623(t322: f64, t2438: f64, t3382: f64, t3384: f64, t3412: f64, t3413: f64, t3416: f64, t3420: f64, t352: f64, t855: f64, t2292: f64, t255: f64) -> (f64, f64) {
    let t323 = t322 <= 0.0_f64;
    let t331 = t322 <= 0.25e1_f64;
    let t3424 = piecewise5(t323, t3382 + t3384, t331, t3412, -0.21e1_f64 * t3413 * t2438 - 0.105e1_f64 * t855 * t3416 * t352 - 0.1575e1_f64 * t3420 * t2438);
    let t3428 = t2292 * t255;
    (t3424, t3428)
}
