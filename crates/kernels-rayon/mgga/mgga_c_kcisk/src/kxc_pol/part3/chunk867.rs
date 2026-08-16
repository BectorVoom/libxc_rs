//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 867/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk867(t12956: f64, t13014: f64, t1191: f64, t1172: f64, t1170: f64, t3675: f64, t305: f64, t320: f64, t3678: f64, t12911: f64, t12974: f64, t12922: f64, t12927: f64, t12929: f64, t12931: f64, t12933: f64, t12948: f64, t12954: f64, t12959: f64, t12985: f64, t12989: f64) -> (f64, f64, f64) {
    let t13015 = t12956 + t13014;
    let t13016 = t13015 * t1191;
    let t13018 = 1.0_f64 * t1172 * t13016;
    let t13020 = 1.0_f64 / t3675 / t1170;
    let t13021 = t305 * t13020;
    let t13023 = 1.0_f64 / t3678 / t320;
    let t13024 = t12911 * t13023;
    let t13026 = 0.51725014705706168417e3_f64 * t13021 * t13024;
    let t13027 = 0.28842592592592592592e-1_f64 * t12974;
    let t13038 = -t13027 - 0.12361111111111111111e-1_f64 * t12929 + 0.61805555555555555556e-2_f64 * t12933 - 0.18541666666666666667e-1_f64 * t12948 + 0.92708333333333333334e-2_f64 * t12931 - 0.10300925925925925926e-1_f64 * t12922 + 0.37083333333333333333e-1_f64 * t12954 - 0.18541666666666666666e-1_f64 * t12985 - 0.55625000000000000001e-1_f64 * t12959 + 0.55625000000000000001e-1_f64 * t12989 - 0.92708333333333333333e-2_f64 * t12927;
    (t13018, t13026, t13038)
}
