//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 918/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk918(t13953: f64, t950: f64, t931: f64, t13712: f64, t13714: f64, t13710: f64, t13717: f64, t13720: f64, t13723: f64, t13726: f64, t13729: f64, t13732: f64, t13735: f64, t13738: f64, t13742: f64, t9681: f64, t9683: f64, t9691: f64, t9700: f64, t9775: f64) -> (f64, f64) {
    let t13954 = t13953 * t950;
    let t13956 = 1.0_f64 * t931 * t13954;
    let t13962 = 0.41203703703703703704e-2_f64 * t13712;
    let t13963 = 0.12361111111111111111e-1_f64 * t13714;
    let t13973 = -t9775 - 0.82407407407407407407e-2_f64 * t9691 + 0.20601851851851851852e-2_f64 * t9683 - 0.61805555555555555556e-2_f64 * t9700 + 0.30902777777777777778e-2_f64 * t9681 - 0.41203703703703703704e-2_f64 * t13710 + t13962 - t13963 + 0.67986111111111111113e-1_f64 * t13717 - 0.10300925925925925926e-1_f64 * t13720 + 0.37083333333333333333e-1_f64 * t13723 - 0.24722222222222222222e-1_f64 * t13726 - 0.61805555555555555555e-2_f64 * t13729 - 0.55625000000000000001e-1_f64 * t13732 + 0.74166666666666666668e-1_f64 * t13735 + 0.18541666666666666667e-1_f64 * t13738 - 0.18541666666666666667e-1_f64 * t13742;
    (t13956, t13973)
}
