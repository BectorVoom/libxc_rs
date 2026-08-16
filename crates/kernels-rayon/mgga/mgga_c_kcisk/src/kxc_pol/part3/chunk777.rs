//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 777/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk777(t2028: f64, t5437: f64, t5438: f64, t791: f64, t10423: f64, t10429: f64, t10434: f64, t10439: f64, t10445: f64, t10453: f64, t10456: f64, t10467: f64, t10469: f64, t10474: f64, t10477: f64, t10482: f64, t10484: f64, t10491: f64, t10495: f64, t10497: f64, t5348: f64, t5445: f64, t5521: f64) -> (f64, f64) {
    let t11964 = t5437 * t2028;
    let t11966 = 1.0_f64 / t5438 / t791;
    let t11967 = t11964 * t11966;
    let t11981 = 0.46429444444444444443e-2_f64 * t10423 - 0.579e0_f64 * t5348 * t5521 - 0.46429444444444444443e-2_f64 * t10429 - 0.34822083333333333333e-2_f64 * t10434 - 0.11607361111111111111e-2_f64 * t10439 + 0.69644166666666666666e-2_f64 * t10445 - 0.223494e0_f64 * t5445 * t11967 + 0.11607361111111111111e-2_f64 * t10453 + 0.58036805555555555555e-2_f64 * t10456 + 0.51588271604938271605e-2_f64 * t10467 - 0.46429444444444444443e-2_f64 * t10469 - 0.77382407407407407405e-3_f64 * t10474 - 0.69644166666666666666e-2_f64 * t10477 + 0.11607361111111111111e-2_f64 * t10482 - 0.46429444444444444443e-2_f64 * t10484 - 0.11607361111111111111e-1_f64 * t10491 + 0.38691203703703703703e-2_f64 * t10495 + 0.23214722222222222222e-2_f64 * t10497;
    (t11967, t11981)
}
