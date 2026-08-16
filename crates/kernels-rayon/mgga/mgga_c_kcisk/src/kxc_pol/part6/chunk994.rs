//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 994/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk994(t13715: f64, t30391: f64, t4129: f64, t20292: f64, t20373: f64, t26138: f64, t26150: f64, t26159: f64, t26176: f64, t26179: f64, t30288: f64, t30292: f64, t30296: f64, t30300: f64, t30303: f64, t30327: f64, t30340: f64) -> (f64, f64) {
    let t30403 = t13715 * t30391;
    let t30404 = t30403 * t4129;
    let t30421 = -0.60384999999999999999e0_f64 * t30296 + 0.181155e1_f64 * t30303 - 0.5519e0_f64 * t20373 - 0.40256666666666666668e0_f64 * t20292 - 0.412621875e-1_f64 * t30327 + 0.258925e1_f64 * t30340 + 0.11038e0_f64 * t26176 - 0.66228e0_f64 * t26179 - 0.60385000000000000001e0_f64 * t26150 + 0.30192500000000000001e0_f64 * t26159 + 0.20128333333333333333e0_f64 * t26138 - 0.33547222222222222222e0_f64 * t30288 + 0.12077e1_f64 * t30292 - 0.181155e1_f64 * t30300;
    (t30404, t30421)
}
