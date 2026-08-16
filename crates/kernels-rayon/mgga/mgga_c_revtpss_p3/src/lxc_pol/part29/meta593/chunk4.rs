//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1981/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1981(t98235: f64, t98238: f64, t98243: f64, t94485: f64, t94498: f64, t94501: f64, t94503: f64, t94505: f64, t94509: f64, t94511: f64, t96326: f64, t98245: f64, t98253: f64) -> f64 {
    let t102534 = 0.22866142996303859718e-3_f64 * t98235;
    let t102535 = 0.57165357490759649296e-4_f64 * t98238;
    let t102537 = 0.2032800112371413129e-3_f64 * t98243;
    let t102546 = -t102534 + t102535 + t96326 + 7.0_f64 / 72.0_f64 * t94485 + t102537 + 0.68598428988911579156e-2_f64 * t98245 + 0.10841600599314203355e-2_f64 * t94498 - 0.22866142996303859718e-3_f64 * t94501 + 0.40015750243531754507e-2_f64 * t94503 + 0.40015750243531754507e-2_f64 * t94505 + 0.10164000561857065645e-3_f64 * t94509 - 0.50820002809285328225e-4_f64 * t94511 - t98253 / 24.0_f64;
    t102546
}
