//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1097/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1097(t1073: f64, t12112: f64, t17567: f64, t21058: f64, t2258: f64, t2259: f64, t2265: f64, t2271: f64, t48442: f64, t4883: f64, t631: f64, t632: f64, t637: f64, t65113: f64, t72: f64, t76221: f64, t76232: f64, t76238: f64, t76241: f64, t76265: f64, t76302: f64, t85451: f64, t85469: f64, t85501: f64, t8660: f64) -> f64 {
    let t87941 = 12.0_f64 * t76221 + 12.0_f64 * t2265 * t12112 * t21058 - 16.0_f64 / 81.0_f64 * t76232 + 8.0_f64 / 9.0_f64 * t76238 - 10.0_f64 * t65113 - 16.0_f64 * t76241 - 4.0_f64 * t631 * t72 * t8660 * t85469 - t631 * t72 * t2271 * t85451 + t631 * t2258 * t2259 * t85451 / 6.0_f64 + t631 * t72 * t632 * t85501 / 6.0_f64 + 36.0_f64 * t631 * t637 * t17567 * t4883 - 160.0_f64 / 27.0_f64 * t48442 - 4.0_f64 / 3.0_f64 * t76265 - 6.0_f64 * t631 * t637 * t76302 * t1073;
    t87941
}
