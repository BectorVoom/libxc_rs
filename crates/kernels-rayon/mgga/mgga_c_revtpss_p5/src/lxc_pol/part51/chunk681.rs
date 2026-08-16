//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 681/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk681(t239: f64, t7262: f64, t820: f64, t1401: f64, t1405: f64, t2019: f64, t545: f64, t64: f64, t1416: f64, t7251: f64, t7253: f64, t7258: f64, t7261: f64) -> (f64, f64, f64, f64, f64) {
    let t7264 = t820 * t7262 * t239;
    let t7265 = t7264 * t1401;
    let t7267 = t2019 * t1405;
    let t7268 = 0.20007875121765877254e-2_f64 * t7267;
    let t7269 = t545 * t64;
    let t7271 = t820 * t7269 * t239;
    let t7272 = t7271 * t1416;
    let t7274 = -t7251 - t7253 / 48.0_f64 - t7258 + t7261 - 0.42874018118069736972e-3_f64 * t7265 - t7268 - 0.17149607247227894789e-2_f64 * t7272;
    (t7264, t7268, t7269, t7271, t7274)
}
