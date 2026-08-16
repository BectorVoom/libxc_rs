//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2191/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2191(t1907: f64, t5591: f64, t25082: f64, t8717: f64, t29495: f64, t7235: f64, t5778: f64, t28196: f64, t28197: f64, t28184: f64, t7898: f64, t5920: f64, t648: f64) -> (f64, f64, f64, f64, f64) {
    let t108682 = t5591 * t1907;
    let t108685 = 6.0_f64 * t25082 * t8717 * t108682;
    let t108687 = 3.0_f64 * t7235 * t29495;
    let t108688 = t1907 * t5778;
    let t108691 = 4.0_f64 * t28196 * t28197 * t108688;
    let t108693 = 6.0_f64 * t7898 * t28184;
    let t108710 = t648 * t5920;
    (t108685, t108687, t108691, t108693, t108710)
}
