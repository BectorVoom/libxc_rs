//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 473/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk473(t322: f64, t2393: f64, t1035: f64, t1348: f64, t2406: f64, t2408: f64, t2436: f64, t2437: f64, t2438: f64, t352: f64, t855: f64, t1357: f64, t457: f64, t898: f64) -> (f64, f64, f64, f64, f64) {
    let t323 = t322 <= 0.0_f64;
    let t331 = t322 <= 0.25e1_f64;
    let t332 = 0.25e1_f64 < t322;
    let t2441 = piecewise3(t332, t2393, 0.0_f64);
    let t2445 = t1348 * t1035;
    let t2449 = piecewise5(t323, t2406 + t2408, t331, t2436, -0.21e1_f64 * t2437 * t2438 - 0.105e1_f64 * t855 * t2441 * t352 - 0.1575e1_f64 * t2445 * t2438);
    let t2451 = 4.0_f64 * t1357;
    let t2452 = t898 * t457;
    (t2441, t2445, t2449, t2451, t2452)
}
