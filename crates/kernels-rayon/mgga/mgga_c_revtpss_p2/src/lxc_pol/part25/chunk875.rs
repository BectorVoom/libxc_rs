//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 875/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk875(t10355: f64, t10356: f64, t2275: f64, t606: f64, t2258: f64, t10326: f64, t48: f64, t58: f64, t59: f64, t2282: f64, t60: f64, t10199: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10357 = t10355 * t10356;
    let t10360 = t2275 * t606;
    let t10361 = t10360 * t2258;
    let t10364 = t48 * t10326;
    let t10368 = 1.0_f64 / t59 / t58;
    let t10369 = t10368 * t10356;
    let t10372 = t2282 * t606;
    let t10373 = t10372 * t2258;
    let t10376 = t60 * t10326;
    let t10379 = 1232.0_f64 / 27.0_f64 * t10199;
    (t10357, t10361, t10364, t10369, t10373, t10376, t10379)
}
