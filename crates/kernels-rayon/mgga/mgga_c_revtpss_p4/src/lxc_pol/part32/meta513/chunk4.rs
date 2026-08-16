//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1813/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1813(t114: f64, t2089: f64, t5920: f64, t2055: f64, t6765: f64, t26148: f64, t28034: f64, t29999: f64, t30001: f64) -> (f64, f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t30558 = t2089 * t5920;
    let t30563 = t6765 * t2055;
    let t30570 = piecewise3(t115, 0.0_f64, t26148 + 4.0_f64 / 3.0_f64 * t28034 + t29999 / 2.0_f64 - t30001 / 4.0_f64);
    (t30558, t30563, t30570)
}
