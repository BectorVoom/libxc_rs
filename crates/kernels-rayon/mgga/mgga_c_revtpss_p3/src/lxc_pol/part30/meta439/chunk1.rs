//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1688/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1688(t3636: f64, t5381: f64, t1260: f64, t12966: f64, t16775: f64, t247: f64, t3719: f64, t1222: f64, t1261: f64, t17232: f64, t17237: f64, t17243: f64, t17244: f64, t17247: f64, t17250: f64, t17254: f64, t17258: f64, t5384: f64, t5386: f64) -> (f64, f64) {
    let t17260 = 0.19055119163586549765e-3_f64 * t5381 * t3636;
    let t17261 = t12966 * t1260;
    let t17265 = t247 * t3719 * t16775;
    let t17268 = -0.57165357490759649296e-3_f64 * t1261 * t17232 - 0.63517063878621832552e-3_f64 * t1261 * t17237 - t17243 - t1222 * t17244 / 72.0_f64 - t1222 * t17247 / 144.0_f64 - t1222 * t17250 / 48.0_f64 + 0.85748036236139473944e-3_f64 * t5384 * t17254 + t17258 - t17260 + 0.85748036236139473944e-3_f64 * t17261 * t5386 + 0.42874018118069736972e-3_f64 * t5384 * t17265;
    (t17265, t17268)
}
