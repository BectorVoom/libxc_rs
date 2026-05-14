//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1152/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1152<F: Float>(t1335: F, t21290: F, t1316: F, t11576: F, t6989: F, t11409: F, t11520: F, t16046: F, t16052: F, t16232: F, t16233: F, t21186: F, t21188: F, t21193: F, t21196: F, t21206: F, t21209: F, t21212: F, t21234: F, t21237: F, t21240: F, t21243: F) -> (F, F, F) {
    let t21291 = t21290 * t1335;
    let t21293 = 1.0 * t1316 * t21291;
    let t21295 = 0.16081824322151104822e2 * t11576 * t6989;
    let t21310 = -t11520 - 0.41203703703703703703e-2 * t11409 - 0.82407407407407407408e-2 * t16046 + t16232 - t16233 - 0.12361111111111111111e-1 * t16052 + 0.20601851851851851852e-2 * t21186 - 0.10300925925925925926e-1 * t21237 + 0.37083333333333333333e-1 * t21234 + 0.24722222222222222222e-1 * t21240 - 0.61805555555555555557e-2 * t21188 - 0.55625000000000000001e-1 * t21243 - 0.74166666666666666668e-1 * t21206 + 0.30902777777777777778e-2 * t21196 - 0.61805555555555555555e-2 * t21209 + 0.18541666666666666667e-1 * t21212 - 0.92708333333333333333e-2 * t21193;
    (t21293, t21295, t21310)
}
