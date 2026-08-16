//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1911/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1911(t98229: f64, t98235: f64, t98238: f64, t98243: f64, t98258: f64, t98269: f64, t98281: f64, t1904: f64, t2439: f64, t26358: f64, t213: f64, t28888: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t102531 = 0.22866142996303859718e-3_f64 * t98229;
    let t102534 = 0.22866142996303859718e-3_f64 * t98235;
    let t102535 = 0.57165357490759649296e-4_f64 * t98238;
    let t102537 = 0.2032800112371413129e-3_f64 * t98243;
    let t102548 = 0.11433071498151929859e-3_f64 * t98258;
    let t102557 = 7.0_f64 / 36.0_f64 * t98269;
    let t102567 = 0.22866142996303859718e-3_f64 * t98281;
    let t102582 = t2439 * t26358 * t1904;
    let t102594 = t213 * t28888;
    (t102531, t102534, t102535, t102537, t102548, t102557, t102567, t102582, t102594)
}
