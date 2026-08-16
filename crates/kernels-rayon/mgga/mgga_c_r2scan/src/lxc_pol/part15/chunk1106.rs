//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1106/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1106(t11856: f64, t3270: f64, t3269: f64, t10940: f64, t11545: f64, t1234: f64, t3574: f64, t10610: f64, t3263: f64, t10619: f64, t11523: f64, t2259: f64, t2867: f64) -> (f64, f64, f64, f64, f64) {
    let t39274 = t3270 * t11856;
    let t39276 = t3269 * t39274 / 2.0_f64;
    let t39278 = 5.0_f64 / 16.0_f64 * t10940 * t11545;
    let t39279 = t3574 * t1234;
    let t39282 = 3.0_f64 / 2.0_f64 * t10610 * t3263 * t39279;
    let t39284 = t11523 * t10619 / 2.0_f64;
    let t39286 = t2867 * t2259;
    (t39276, t39278, t39282, t39284, t39286)
}
