//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1289/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1289(t1335: f64, t21290: f64, t1316: f64, t11576: f64, t6989: f64, t11409: f64, t11520: f64, t16046: f64, t16052: f64, t16232: f64, t16233: f64, t21186: f64, t21188: f64, t21193: f64, t21196: f64, t21206: f64, t21209: f64, t21212: f64, t21234: f64, t21237: f64, t21240: f64, t21243: f64) -> (f64, f64, f64) {
    let t21291 = t21290 * t1335;
    let t21293 = 1.0_f64 * t1316 * t21291;
    let t21295 = 0.16081824322151104822e2_f64 * t11576 * t6989;
    let t21310 = -t11520 - 0.41203703703703703703e-2_f64 * t11409 - 0.82407407407407407408e-2_f64 * t16046 + t16232 - t16233 - 0.12361111111111111111e-1_f64 * t16052 + 0.20601851851851851852e-2_f64 * t21186 - 0.10300925925925925926e-1_f64 * t21237 + 0.37083333333333333333e-1_f64 * t21234 + 0.24722222222222222222e-1_f64 * t21240 - 0.61805555555555555557e-2_f64 * t21188 - 0.55625000000000000001e-1_f64 * t21243 - 0.74166666666666666668e-1_f64 * t21206 + 0.30902777777777777778e-2_f64 * t21196 - 0.61805555555555555555e-2_f64 * t21209 + 0.18541666666666666667e-1_f64 * t21212 - 0.92708333333333333333e-2_f64 * t21193;
    (t21293, t21295, t21310)
}
