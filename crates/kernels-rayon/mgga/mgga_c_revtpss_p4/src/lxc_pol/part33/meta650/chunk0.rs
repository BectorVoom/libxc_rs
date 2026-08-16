//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2101/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2101(t17445: f64, t7607: f64, t3655: f64, t8177: f64, t1256: f64, t29074: f64, t29069: f64, t29089: f64, t3685: f64, t26948: f64, t97065: f64, t3555: f64, t8190: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t104994 = t7607 * t17445 / 432.0_f64;
    let t104999 = t8177 * t3655;
    let t105002 = 0.57165357490759649296e-3_f64 * t29074 * t1256;
    let t105007 = 0.30488190661738479624e-2_f64 * t29069 * t1256;
    let t105014 = t29089 * t3685 / 162.0_f64;
    let t105046 = t26948 * t97065;
    let t105134 = t3555 * t8190;
    (t104994, t104999, t105002, t105007, t105014, t105046, t105134)
}
