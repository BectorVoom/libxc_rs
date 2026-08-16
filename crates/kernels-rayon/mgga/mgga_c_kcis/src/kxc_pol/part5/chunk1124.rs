//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1124/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1124(t18944: f64, t950: f64, t931: f64, t6393: f64, t9655: f64, t13710: f64, t13717: f64, t13962: f64, t13963: f64, t18645: f64, t18650: f64, t18655: f64, t18659: f64, t18661: f64, t18664: f64, t18667: f64, t18669: f64, t18674: f64, t18679: f64, t18683: f64, t9691: f64, t9775: f64) -> (f64, f64, f64) {
    let t18945 = t18944 * t950;
    let t18947 = 1.0_f64 * t931 * t18945;
    let t18949 = 0.16081824322151104822e2_f64 * t9655 * t6393;
    let t18964 = -t9775 - 0.41203703703703703703e-2_f64 * t9691 - 0.82407407407407407408e-2_f64 * t13710 + t13962 - t13963 + 0.12361111111111111111e-1_f64 * t13717 + 0.20601851851851851852e-2_f64 * t18645 - 0.10300925925925925926e-1_f64 * t18650 + 0.37083333333333333333e-1_f64 * t18655 - 0.24722222222222222222e-1_f64 * t18659 - 0.61805555555555555557e-2_f64 * t18661 - 0.55625000000000000001e-1_f64 * t18664 + 0.74166666666666666668e-1_f64 * t18667 + 0.30902777777777777778e-2_f64 * t18669 - 0.61805555555555555555e-2_f64 * t18674 + 0.18541666666666666667e-1_f64 * t18679 - 0.92708333333333333333e-2_f64 * t18683;
    (t18947, t18949, t18964)
}
