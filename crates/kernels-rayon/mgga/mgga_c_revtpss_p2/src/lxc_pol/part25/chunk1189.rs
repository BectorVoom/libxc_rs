//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1189/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1189(t26021: f64, t7262: f64, t820: f64, t843: f64, t1401: f64, t241: f64) -> (f64, f64, f64, f64) {
    let t26022 = 0.90357964994909313586e-5_f64 * t26021;
    let t26024 = t820 * t7262 * t843;
    let t26025 = t26024 * t1401;
    let t26028 = t820 * t7262 * t241;
    (t26022, t26024, t26025, t26028)
}
