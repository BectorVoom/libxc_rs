//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 392/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk392(t1118: f64, t1143: f64, t1124: f64, t1135: f64, t1140: f64, t1147: f64) -> (f64, f64, f64) {
    let t1163 = 0.516475e0_f64 * t1118;
    let t1166 = 0.104195e0_f64 * t1143;
    let t1168 = 0.3529725e1_f64 * t1135 - t1163 + 0.516475e0_f64 * t1124 + 0.6311625e0_f64 * t1140 - t1166 + 0.104195e0_f64 * t1147;
    (t1163, t1166, t1168)
}
