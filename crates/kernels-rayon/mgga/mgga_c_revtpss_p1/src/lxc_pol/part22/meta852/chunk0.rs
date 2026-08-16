//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2993/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2993(t4186: f64, t4401: f64, t606: f64, t749: f64, t14362: f64, t9575: f64, t123: f64, t2630: f64, t4392: f64, t4398: f64, t9318: f64, t15071: f64, t892: f64) -> (f64, f64, f64, f64, f64) {
    let t49911 = t4401 * t749 * t4186 * t606;
    let t49926 = t14362 * t9575;
    let t49929 = t4392 * t123 * t2630;
    let t49940 = t4398 * t9318;
    let t49950 = t15071 * t892;
    (t49911, t49926, t49929, t49940, t49950)
}
