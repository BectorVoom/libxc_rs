//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1917/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1917(t2435: f64, t8011: f64, t25431: f64, t2439: f64, t93170: f64, t28347: f64, t686: f64, t72: f64, t25387: f64, t102980: f64, t93190: f64, t10073: f64, t26554: f64, t27198: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t102993 = t8011 * t2435;
    let t102994 = t25431 * t102993;
    let t103000 = t8011 * t2439;
    let t103001 = t93170 * t103000;
    let t103005 = t28347 * t72 * t686;
    let t103007 = 0.51405703062096148812e-1_f64 * t25387 * t103005;
    let t103009 = t93190 * t102980;
    let t103017 = t10073 * t27198 * t26554;
    (t102993, t102994, t103000, t103001, t103005, t103007, t103009, t103017)
}
