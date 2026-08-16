//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 810/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk810(t9259: f64, t9262: f64, t9214: f64, t9217: f64, t9220: f64, t9224: f64, t9226: f64, t9230: f64, t9233: f64, t9235: f64, t9239: f64, t9242: f64, t9250: f64, t9257: f64) -> (f64, f64) {
    let t9263 = t9259 * t9262;
    let t9265 = 0.86880925264517213544e-4_f64 * t9214 - 0.14480154210752868924e-5_f64 * t9217 - 0.25745714186718600948e-5_f64 * t9220 - 0.25745714186718600948e-5_f64 * t9224 + 0.10821235962619981449e-3_f64 * t9226 + 0.20241536458333333334e-4_f64 * t9230 + 0.10120768229166666667e-3_f64 * t9233 - 0.30660168560756614104e-3_f64 * t9235 - 0.11101451561577199508e-4_f64 * t9239 - 0.10120768229166666667e-4_f64 * t9242 - 0.14591718745976239987e-8_f64 * t9250 + 0.49240895655712845848e-7_f64 * t9257 + 0.98481791311425691697e-7_f64 * t9263;
    (t9263, t9265)
}
