//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1191/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1191(t1416: f64, t7271: f64, t7251: f64, t7253: f64, t7258: f64, t7261: f64, t7265: f64, t7268: f64) -> f64 {
    let t7272 = t7271 * t1416;
    let t7274 = -t7251 - t7253 / 48.0_f64 - t7258 + t7261 - 0.42874018118069736972e-3_f64 * t7265 - t7268 - 0.17149607247227894789e-2_f64 * t7272;
    t7274
}
