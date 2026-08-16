//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1172/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1172(t10719: f64, t8198: f64, t1575: f64, t269: f64, t546: f64, t25968: f64, t39841: f64, t565: f64, t25962: f64, t10728: f64, t7258: f64, t1592: f64, t24786: f64, t3308: f64) -> (f64, f64, f64, f64, f64) {
    let t40059 = t8198 * t10719;
    let t40061 = t1575 * t269;
    let t40062 = t546 * t40061;
    let t40064 = t40062 * t39841 * t25968;
    let t40066 = t565 * t40061;
    let t40068 = t40066 * t39841 * t25962;
    let t40070 = t10728 * t7258;
    let t40073 = t1592 * t3308 * t24786;
    (t40059, t40064, t40068, t40070, t40073)
}
