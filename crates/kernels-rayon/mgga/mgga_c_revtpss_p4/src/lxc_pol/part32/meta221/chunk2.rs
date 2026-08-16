//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 950/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk950(t2339: f64, t5891: f64, t1504: f64, t2349: f64, t100: f64, t5823: f64, t1479: f64, t1509: f64, t2357: f64, t108: f64, t105: f64, t109: f64, t1507: f64, t1510: f64, t97: f64, tau1: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5892 = t2339 * t5891;
    let t5895 = t1504 * t1504;
    let t5896 = t2349 * t5895;
    let t5899 = t100 * t5823;
    let t5902 = tau1 * t1479;
    let t5907 = t1509 * t1509;
    let t5908 = t2357 * t5907;
    let t5911 = -t5823;
    let t5912 = t108 * t5911;
    let t5915 = 10.0_f64 / 9.0_f64 * t97 * t5896 + 5.0_f64 / 3.0_f64 * t97 * t5899 + 40.0_f64 / 9.0_f64 * t5902 * t109 - 50.0_f64 / 9.0_f64 * t1507 * t1510 + 10.0_f64 / 9.0_f64 * t105 * t5908 + 5.0_f64 / 3.0_f64 * t105 * t5912;
    (t5892, t5895, t5896, t5899, t5902, t5907, t5911, t5915)
}
