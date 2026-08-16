//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1475/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1475(t101: f64, t613: f64, t655: f64, t100: f64, t43: f64, t658: f64, t2349: f64, t96: f64, t2350: f64, t2256: f64, t8268: f64, t31026: f64, t31028: f64, t31030: f64, t31033: f64, t31035: f64, t31036: f64, t31040: f64, t31044: f64, t31047: f64, t69: f64, t8258: f64, t8267: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31051 = t655 * t613 * t101;
    let t31054 = t43 * t100;
    let t31055 = t31054 * t658;
    let t31058 = t96 * t2349;
    let t31059 = t31058 * t2350;
    let t31062 = t8268 * t2256;
    let t31065 = -t31026 - 4.0_f64 / 3.0_f64 * t31028 - 10.0_f64 / 9.0_f64 * t31030 + 10.0_f64 / 9.0_f64 * t31033 - 3.0_f64 / 4.0_f64 * t31035 * t31036 - 5.0_f64 / 6.0_f64 * t8258 * t31040 + 5.0_f64 / 6.0_f64 * t8258 * t31044 + t8258 * t31047 / 4.0_f64 - 5.0_f64 / 9.0_f64 * t69 * t31051 + 25.0_f64 / 36.0_f64 * t8267 * t31055 - 5.0_f64 / 36.0_f64 * t8267 * t31059 - 5.0_f64 / 24.0_f64 * t8267 * t31062;
    (t31051, t31054, t31055, t31058, t31059, t31062, t31065)
}
