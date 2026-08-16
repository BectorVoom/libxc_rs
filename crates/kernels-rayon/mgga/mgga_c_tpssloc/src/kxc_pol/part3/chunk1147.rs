//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1147/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1147(t1118: f64, t14913: f64, t1099: f64, t14720: f64, t14722: f64, t14704: f64, t11136: f64, t11137: f64, t11139: f64, t11141: f64, t11143: f64, t14702: f64, t14708: f64, t14728: f64, t14733: f64, t14738: f64, t14742: f64, t14746: f64, t14751: f64, t14755: f64) -> (f64, f64) {
    let t14914 = t14913 * t1118;
    let t14916 = 1.0_f64 * t1099 * t14914;
    let t14922 = 0.41203703703703703704e-2_f64 * t14720;
    let t14923 = 0.12361111111111111111e-1_f64 * t14722;
    let t14924 = 0.61805555555555555556e-2_f64 * t14704;
    let t14933 = -t11136 + 0.82407407407407407407e-2_f64 * t11137 + 0.20601851851851851852e-2_f64 * t11139 - 0.61805555555555555556e-2_f64 * t11141 - 0.30902777777777777778e-2_f64 * t11143 + 0.41203703703703703704e-2_f64 * t14702 + t14922 - t14923 - t14924 + 0.10300925925925925926e-1_f64 * t14728 - 0.37083333333333333333e-1_f64 * t14733 - 0.12361111111111111111e-1_f64 * t14738 - 0.61805555555555555555e-2_f64 * t14742 + 0.55625000000000000001e-1_f64 * t14746 + 0.37083333333333333334e-1_f64 * t14751 + 0.18541666666666666667e-1_f64 * t14755 + 0.92708333333333333333e-2_f64 * t14708;
    (t14916, t14933)
}
