//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1496/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1496(t10199: f64, t2339: f64, t2: f64, t665: f64, t101457: f64, t116919: f64, t116946: f64, t1504: f64, t1513: f64, t2256: f64, t2340: f64, t2350: f64, t28036: f64, t31035: f64, t31039: f64, t31054: f64, t31058: f64, t31267: f64, t31276: f64, t31287: f64, t4287: f64, t658: f64, t8258: f64, t8259: f64, t8267: f64, t8268: f64) -> f64 {
    let t117544 = t10199 * t2339;
    let t117545 = t2 * t665;
    let t117560 = -25.0_f64 / 18.0_f64 * t8258 * t31054 * t31267 + 5.0_f64 / 6.0_f64 * t8258 * t8268 * t4287 * t658 + 5.0_f64 / 12.0_f64 * t8258 * t8268 * t1513 * t2256 + 5.0_f64 / 2.0_f64 * t31035 * t31039 * t28036 + 5.0_f64 / 18.0_f64 * t8258 * t31058 * t1513 * t2350 - 5.0_f64 / 4.0_f64 * t31035 * t8268 * t1504 * t2340 - 25.0_f64 / 18.0_f64 * t8258 * t31054 * t31276 + 5.0_f64 / 6.0_f64 * t117544 * t8268 * t117545 + 5.0_f64 / 108.0_f64 * t8267 * t116946 * t1504 * t2350 - 5.0_f64 / 18.0_f64 * t31287 * t31058 * t2 * t658 + 3.0_f64 * t116919 * t8259 * t101457;
    t117560
}
