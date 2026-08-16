//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 989/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk989(t1235: f64, t30339: f64, t20292: f64, t20373: f64, t26138: f64, t26150: f64, t26159: f64, t26176: f64, t26179: f64, t30288: f64, t30292: f64, t30296: f64, t30300: f64, t30303: f64, t30327: f64) -> (f64, f64) {
    let t30340 = t1235 * t30339;
    let t30350 = -0.59793333333333333333e0_f64 * t30296 + 0.17938e1_f64 * t30303 - 0.5477111111111111111e0_f64 * t20373 - 0.39862222222222222223e0_f64 * t20292 - 0.76790625e-1_f64 * t30327 + 0.1898925e1_f64 * t30340 + 0.10954222222222222222e0_f64 * t26176 - 0.65725333333333333332e0_f64 * t26179 - 0.59793333333333333333e0_f64 * t26150 + 0.29896666666666666667e0_f64 * t26159 + 0.19931111111111111111e0_f64 * t26138 - 0.33218518518518518518e0_f64 * t30288 + 0.11958666666666666667e1_f64 * t30292 - 0.17938e1_f64 * t30300;
    (t30340, t30350)
}
