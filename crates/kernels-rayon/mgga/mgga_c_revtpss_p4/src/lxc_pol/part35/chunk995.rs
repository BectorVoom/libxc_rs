//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 995/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk995(t17023: f64, t17032: f64, t17154: f64, t24219: f64, t24223: f64, t24253: f64, t24257: f64, t24259: f64, t24261: f64, t24264: f64, t24326: f64, t24329: f64, t24431: f64, t24436: f64, t24453: f64, t24468: f64, t3477: f64, t3521: f64, t435: f64, t5120: f64, t6487: f64, t6503: f64, t6506: f64, t6519: f64) -> f64 {
    let t24470 = -6.0_f64 * t17023 * t6487 + 6.0_f64 * t3477 * t24431 - 0.35089341735807877242e1_f64 * t17154 * t6519 + 0.35089341735807877242e1_f64 * t3521 * t24436 + t24219 - t24223 - t24257 - t24259 - t24261 + t24264 - t24326 - t24329 + 3.0_f64 * t5120 * t6503 + 0.96491876992155210402e2_f64 * t17032 * t6506 - 0.310907e-1_f64 * t24453 * t435 + t24468 - 0.19751673498613801407e-1_f64 * t24253;
    t24470
}
