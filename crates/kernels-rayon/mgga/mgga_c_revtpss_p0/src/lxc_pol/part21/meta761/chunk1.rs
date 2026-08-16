//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2697/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2697(t1868: f64, t4135: f64, t13586: f64, t3889: f64, t39799: f64, t4139: f64, t47059: f64, t48265: f64, t48266: f64, t48268: f64, t48270: f64, t48271: f64, t48275: f64, t5536: f64, t5537: f64, t7315: f64, t9628: f64) -> f64 {
    let t49582 = t1868 * t4135;
    let t49592 = 18.0_f64 * t13586 * t3889 * t5536 - 9.0_f64 * t4139 * t49582 * t7315 + 6.0_f64 * t5536 * t5537 * t9628 + t39799 + t47059 - t48265 - t48266 + t48268 - t48270 - t48271 + t48275;
    t49592
}
