//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2814/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2814(t2439: f64, t2440: f64, t2829: f64, t2410: f64, t2985: f64, t3010: f64, t3013: f64, t241: f64, t281: f64, t283: f64, t2297: f64, t2851: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41125 = t2439 * t2440 * t2829;
    let t41153 = t2410 * t2410;
    let t41154 = 1.0_f64 / t41153;
    let t41224 = 1.0_f64 / t3010 / t2985;
    let t41234 = t3010 * t3010;
    let t41235 = 1.0_f64 / t41234;
    let t41237 = t3013 * t3013;
    let t41238 = 1.0_f64 / t41237;
    let t41245 = t281 * t241 * t283;
    let t41246 = 0.13490888888888888889e1_f64 * t41245;
    let t41270 = 1.0_f64 / t2851 / t2297;
    (t41125, t41154, t41224, t41235, t41238, t41245, t41246, t41270)
}
