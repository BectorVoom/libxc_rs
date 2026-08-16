//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2242/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2242(t17445: f64, t7607: f64, t3655: f64, t8177: f64, t1256: f64, t29074: f64, t29069: f64, t29089: f64, t3685: f64, t17332: f64, t17405: f64, t2138: f64, t3650: f64, t3689: f64, t3694: f64, t3701: f64, t484: f64, t8184: f64) -> f64 {
    let t104994 = t7607 * t17445 / 432.0_f64;
    let t104999 = t8177 * t3655;
    let t105002 = 0.57165357490759649296e-3_f64 * t29074 * t1256;
    let t105007 = 0.30488190661738479624e-2_f64 * t29069 * t1256;
    let t105014 = t29089 * t3685 / 162.0_f64;
    let t105017 = -t104994 + t29089 * t3689 / 108.0_f64 + t29089 * t3694 / 54.0_f64 - 0.95275595817932748827e-4_f64 * t104999 + t105002 + 0.42874018118069736972e-3_f64 * t17332 * t2138 * t484 - t105007 - 0.22866142996303859718e-2_f64 * t3650 * t8184 * t484 - t29089 * t3701 / 81.0_f64 + t105014 - t7607 * t17405 / 288.0_f64;
    t105017
}
