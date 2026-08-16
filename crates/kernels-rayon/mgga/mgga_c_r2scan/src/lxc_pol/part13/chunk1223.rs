//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1223/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1223(t10669: f64, t40681: f64, t10630: f64, t11479: f64, t3262: f64, t3617: f64, t5086: f64, t10998: f64, t3275: f64, t797: f64, t8296: f64, t3276: f64) -> (f64, f64, f64, f64) {
    let t40683 = 3.0_f64 / 2.0_f64 * t40681 * t10669;
    let t40686 = 3.0_f64 / 4.0_f64 * t3262 * t11479 * t10630;
    let t40687 = t5086 * t3617;
    let t40690 = 45.0_f64 / 64.0_f64 * t3275 * t40687 * t10998;
    let t40691 = t797 * t8296;
    let t40694 = 5.0_f64 / 16.0_f64 * t3275 * t3276 * t40691;
    (t40683, t40686, t40690, t40694)
}
