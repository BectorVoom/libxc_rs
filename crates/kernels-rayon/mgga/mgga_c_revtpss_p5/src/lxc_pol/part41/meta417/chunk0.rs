//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1469/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1469(t31276: f64, t8268: f64, t1504: f64, t31054: f64, t658: f64, t31058: f64, t10199: f64, t655: f64, t2: f64, t31026: f64, t31028: f64, t31030: f64, t31033: f64, t31035: f64, t31259: f64, t31261: f64, t31264: f64, t31268: f64, t31271: f64, t31274: f64, t8258: f64, t8267: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31277 = t8268 * t31276;
    let t31280 = t31054 * t1504;
    let t31283 = t1504 * t658;
    let t31284 = t31058 * t31283;
    let t31287 = t10199 * t655;
    let t31288 = t8268 * t2;
    let t31291 = -t31026 - 2.0_f64 / 3.0_f64 * t31028 - 5.0_f64 / 9.0_f64 * t31030 + 5.0_f64 / 9.0_f64 * t31033 - 2.0_f64 / 3.0_f64 * t31259 - 3.0_f64 / 4.0_f64 * t31035 * t31261 - 5.0_f64 / 12.0_f64 * t8258 * t31264 + 5.0_f64 / 12.0_f64 * t8258 * t31268 + t8258 * t31271 / 4.0_f64 + 5.0_f64 / 9.0_f64 * t31274 + 5.0_f64 / 12.0_f64 * t8258 * t31277 + 25.0_f64 / 72.0_f64 * t8267 * t31280 - 5.0_f64 / 36.0_f64 * t8267 * t31284 - 5.0_f64 / 24.0_f64 * t31287 * t31288;
    (t31277, t31280, t31283, t31284, t31287, t31288, t31291)
}
