//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 698/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk698(t3365: f64, t3366: f64, t3255: f64, t532: f64, t1219: f64, t1253: f64, t1233: f64, t1260: f64, t220: f64, t3261: f64, t3327: f64, t3332: f64, t3357: f64, t339: f64, t523: f64) -> (f64, f64, f64) {
    let t3367 = t3365 * t3366;
    let t3370 = t3255 * t532;
    let t3374 = t1219 * t1253;
    let t3384 = -2.0_f64 * t1233 * t3374 * t339 - t1260 * t3327 * t339 - t1260 * t3332 * t339 + t220 * t3357 * t523 + 2.0_f64 * t3261 * t3370 * t339;
    (t3367, t3374, t3384)
}
