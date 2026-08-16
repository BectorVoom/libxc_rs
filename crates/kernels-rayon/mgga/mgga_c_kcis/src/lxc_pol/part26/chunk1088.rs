//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1088/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1088(t16937: f64, t8154: f64, t7908: f64, t1497: f64, t15955: f64, t27387: f64, t1464: f64, t1938: f64, t3717: f64, t1385: f64, t27370: f64, t1380: f64, t5885: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28335 = t16937 * t8154;
    let t28336 = t7908 * t28335;
    let t28338 = t15955 * t1497;
    let t28339 = t27387 * t28338;
    let t28340 = t1464 * t28339;
    let t28342 = t3717 * t1938;
    let t28343 = t28342 * t1385;
    let t28344 = t27370 * t28343;
    let t28347 = t5885 * t1380;
    (t28335, t28336, t28338, t28339, t28340, t28342, t28343, t28344, t28347)
}
