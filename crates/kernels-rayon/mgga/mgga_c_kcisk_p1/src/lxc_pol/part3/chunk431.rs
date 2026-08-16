//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 431/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk431(t3310: f64, t242: f64, t1077: f64, t1078: f64, t2864: f64, t2867: f64, t2869: f64, t2873: f64, t2875: f64, t2877: f64, t1070: f64, t250: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3311 = 1.0_f64 / t3310;
    let t3312 = t242 * t3311;
    let t3313 = t1077 * t1077;
    let t3314 = t3313 * t1078;
    let t3323 = -0.78438333333333333333e0_f64 * t2864 + 0.15687666666666666667e1_f64 * t2867 + 0.68863333333333333333e0_f64 * t2869 + 0.14025833333333333333e0_f64 * t2873 + 0.28051666666666666667e0_f64 * t2875 + 0.17365833333333333333e0_f64 * t2877;
    let t3324 = t3323 * t1078;
    let t3327 = t1070 * t1070;
    let t3328 = 1.0_f64 / t3327;
    let t3329 = t242 * t3328;
    let t3330 = t250 * t250;
    (t3311, t3312, t3313, t3314, t3323, t3324, t3327, t3328, t3329, t3330)
}
