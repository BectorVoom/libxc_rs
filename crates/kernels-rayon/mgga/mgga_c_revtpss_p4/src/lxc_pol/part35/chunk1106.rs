//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1106/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1106(t11006: f64, t256: f64, t10115: f64, t251: f64, t2410: f64, t90: f64, t29: f64, t560: f64, t9655: f64, t1389: f64, t268: f64, t555: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41077 = 1.0_f64 / t11006 / t256;
    let t41117 = t10115 * t251;
    let t41153 = t2410 * t2410;
    let t41154 = 1.0_f64 / t41153;
    let t45970 = t90 * t90;
    let t45972 = t29 / t45970;
    let t46361 = 1.0_f64 / t9655 / t560;
    let t46808 = t1389 * t268;
    let t47567 = t10115 * t555;
    (t41077, t41117, t41154, t45972, t46361, t46808, t47567)
}
