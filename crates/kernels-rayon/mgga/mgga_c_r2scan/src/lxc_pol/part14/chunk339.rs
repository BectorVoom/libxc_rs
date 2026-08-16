//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 339/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk339(t1267: f64, t1243: f64, t1250: f64, t1253: f64, t1258: f64, t1262: f64, t295: f64, t299: f64, t305: f64, t803: f64, t807: f64, t815: f64, t818: f64) -> (f64, f64, f64) {
    let t1268 = 11.0_f64 / 9.0_f64 * t1267;
    let t1269 = 40.0_f64 / 9.0_f64 * t1243 * t299 - 50.0_f64 / 9.0_f64 * t803 * t807 + 10.0_f64 / 9.0_f64 * t295 * t1250 + 5.0_f64 / 3.0_f64 * t295 * t1253 + 10.0_f64 / 9.0_f64 * t305 * t1258 + 5.0_f64 / 3.0_f64 * t305 * t1262 - t1268;
    let t1271 = t815 * t818;
    (t1268, t1269, t1271)
}
