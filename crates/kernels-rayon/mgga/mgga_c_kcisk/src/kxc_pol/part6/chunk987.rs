//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 987/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk987(t1224: f64, t1225: f64, t30298: f64, t30273: f64, t30238: f64, t13686: f64, t20292: f64, t26138: f64, t26150: f64, t26159: f64, t30288: f64, t30292: f64, t30296: f64) -> (f64, f64, f64, f64) {
    let t30300 = t1224 * t1225 * t30298;
    let t30303 = t1224 * t1225 * t30273;
    let t30306 = t1224 * t1225 * t30238;
    let t30308 = -t13686 - 0.12361111111111111111e-1_f64 * t20292 + 0.61805555555555555556e-2_f64 * t26138 - 0.18541666666666666667e-1_f64 * t26150 + 0.92708333333333333334e-2_f64 * t26159 - 0.10300925925925925926e-1_f64 * t30288 + 0.37083333333333333333e-1_f64 * t30292 - 0.18541666666666666666e-1_f64 * t30296 - 0.55625000000000000001e-1_f64 * t30300 + 0.55625000000000000001e-1_f64 * t30303 - 0.92708333333333333333e-2_f64 * t30306;
    (t30300, t30303, t30306, t30308)
}
