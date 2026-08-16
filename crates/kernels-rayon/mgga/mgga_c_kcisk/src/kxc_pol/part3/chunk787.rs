//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 787/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk787(t12128: f64, t12142: f64, t10508: f64, t10513: f64, t10515: f64, t10517: f64, t10525: f64, t10527: f64, t10530: f64, t10532: f64, t10537: f64, t11209: f64, t11211: f64, t11216: f64, t11967: f64, t11983: f64, t11986: f64, t11991: f64, t1994: f64, t5348: f64, t5440: f64, t5445: f64, t795: f64) -> (f64, f64) {
    let t12143 = t12128 + t12142;
    let t12156 = t11983 - 0.43134342e-1_f64 * t11986 * t11967 + 0.579e0_f64 * t5348 * t5440 + 0.579e0_f64 * t1994 * t11991 + 0.34822083333333333333e-2_f64 * t10508 - 0.52233124999999999998e-2_f64 * t10513 + 0.23214722222222222222e-2_f64 * t10515 + t12143 * t795 - 0.11607361111111111111e-2_f64 * t10517 + 0.11607361111111111111e-2_f64 * t10525 + 0.46429444444444444443e-2_f64 * t10527 - 0.11607361111111111111e-2_f64 * t10530 - 0.34822083333333333333e-2_f64 * t10532 + 0.223494e0_f64 * t5445 * t11991 - 0.34822083333333333333e-2_f64 * t10537 + 0.11607361111111111111e-2_f64 * t11209 - 0.77382407407407407405e-3_f64 * t11211 + 0.51588271604938271604e-3_f64 * t11216;
    (t12143, t12156)
}
