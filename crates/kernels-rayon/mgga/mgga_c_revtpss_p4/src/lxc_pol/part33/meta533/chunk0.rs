//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1885/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1885(t1916: f64, t7331: f64, t7334: f64, t1459: f64, t7950: f64, t1936: f64, t670: f64, t1518: f64, t572: f64, t26123: f64, t4292: f64, t7330: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28259 = 6.0_f64 * t1916 * t7331;
    let t28261 = 3.0_f64 * t1916 * t7334;
    let t28263 = 6.0_f64 * t1459 * t7950;
    let t28264 = t670 * t1936;
    let t28265 = t28264 * t1518;
    let t28267 = 6.0_f64 * t572 * t28265;
    let t28268 = t26123 * t1518;
    let t28270 = 6.0_f64 * t572 * t28268;
    let t28271 = t7330 * t4292;
    (t28259, t28261, t28263, t28264, t28265, t28267, t28268, t28270, t28271)
}
