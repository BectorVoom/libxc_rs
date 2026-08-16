//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1147/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1147(t2439: f64, t8011: f64, t93170: f64, t102980: f64, t93190: f64, t10073: f64, t26554: f64, t27198: f64, t15003: f64, t95773: f64, t26506: f64, t27216: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t103000 = t8011 * t2439;
    let t103001 = t93170 * t103000;
    let t103009 = t93190 * t102980;
    let t103017 = t10073 * t27198 * t26554;
    let t103030 = t95773 * t15003;
    let t103063 = t27216 * t26506;
    (t103000, t103001, t103009, t103017, t103030, t103063)
}
