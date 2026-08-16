//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1088/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1088<F: Float>(t16937: F, t8154: F, t7908: F, t1497: F, t15955: F, t27387: F, t1464: F, t1938: F, t3717: F, t1385: F, t27370: F, t1380: F, t5885: F) -> (F, F, F, F, F, F, F, F, F) {
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
