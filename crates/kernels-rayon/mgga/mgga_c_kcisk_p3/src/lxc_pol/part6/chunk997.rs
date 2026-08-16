//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 997/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk997(t26302: f64, t30451: f64, t13523: f64, t20292: f64, t26138: f64, t26150: f64, t26159: f64, t30288: f64, t30292: f64, t30296: f64, t30300: f64, t30303: f64, t30306: f64) -> (f64, f64) {
    let t30452 = t26302 * t30451;
    let t30465 = -t13523 - 0.23744444444444444444e-1_f64 * t20292 + 0.11872222222222222222e-1_f64 * t26138 - 0.35616666666666666666e-1_f64 * t26150 + 0.17808333333333333333e-1_f64 * t26159 - 0.19787037037037037037e-1_f64 * t30288 + 0.71233333333333333332e-1_f64 * t30292 - 0.35616666666666666666e-1_f64 * t30296 - 0.10685e0_f64 * t30300 + 0.10685e0_f64 * t30303 - 0.17808333333333333333e-1_f64 * t30306;
    (t30452, t30465)
}
