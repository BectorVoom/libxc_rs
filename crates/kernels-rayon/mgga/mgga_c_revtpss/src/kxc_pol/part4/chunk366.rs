//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 366/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk366(t1118: f64, t1143: f64, t1124: f64, t1135: f64, t1140: f64, t1147: f64) -> (f64, f64, f64) {
    let t1182 = 0.301925e0_f64 * t1118;
    let t1185 = 0.82785e-1_f64 * t1143;
    let t1187 = 0.258925e1_f64 * t1135 - t1182 + 0.301925e0_f64 * t1124 + 0.16504875e0_f64 * t1140 - t1185 + 0.82785e-1_f64 * t1147;
    (t1182, t1185, t1187)
}
