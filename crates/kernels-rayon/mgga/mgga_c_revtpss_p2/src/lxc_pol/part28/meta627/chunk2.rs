//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2249/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2249(t28182: f64, t7235: f64, t13392: f64, t603: f64, t13396: f64, t13405: f64, t1928: f64, t25140: f64, t25143: f64, t25147: f64, t28112: f64, t28116: f64, t28119: f64, t6974: f64, t6978: f64, t7709: f64) -> (f64, f64) {
    let t101124 = 2.0_f64 * t7235 * t28182;
    let t101129 = t603 * t13392;
    let t101132 = t603 * t13396;
    let t101139 = t603 * t13405;
    let t101152 = 2.0_f64 / 3.0_f64 * t28112 * t6974 + 2.0_f64 / 3.0_f64 * t28112 * t6978 + t101129 * t1928 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t101132 * t1928 + 2.0_f64 / 3.0_f64 * t28116 * t6974 + 2.0_f64 / 3.0_f64 * t28116 * t6978 + t101139 * t1928 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t28119 * t6974 + 2.0_f64 / 3.0_f64 * t28119 * t6978 + t7709 * t25140 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t7709 * t25143 + t7709 * t25147 / 3.0_f64;
    (t101124, t101152)
}
