//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1122/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1122(t1080: f64, t9176: f64, t15422: f64, t1089: f64, t11938: f64, t11988: f64, t11989: f64, t11990: f64, t15239: f64, t15241: f64, t15243: f64, t15251: f64, t15259: f64, t15264: f64, t15268: f64, t15273: f64, t15277: f64, t15283: f64, t15288: f64, t9221: f64, t9331: f64) -> (f64, f64) {
    let t15423 = t9176 * t1080;
    let t15424 = t15422 * t15423;
    let t15426 = 0.10254018858216406658e4_f64 * t1089 * t15424;
    let t15440 = -t9331 + 0.41203703703703703703e-2_f64 * t9221 + 0.82407407407407407408e-2_f64 * t11938 + t11988 - t11989 - t11990 + 0.20601851851851851852e-2_f64 * t15239 + 0.10300925925925925926e-1_f64 * t15259 - 0.37083333333333333333e-1_f64 * t15264 - 0.12361111111111111111e-1_f64 * t15268 - 0.61805555555555555557e-2_f64 * t15241 + 0.55625000000000000001e-1_f64 * t15273 + 0.37083333333333333334e-1_f64 * t15277 - 0.30902777777777777778e-2_f64 * t15243 - 0.61805555555555555555e-2_f64 * t15283 + 0.18541666666666666667e-1_f64 * t15288 + 0.92708333333333333333e-2_f64 * t15251;
    (t15426, t15440)
}
