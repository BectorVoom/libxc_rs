//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 650/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk650(t1309: f64, t3863: f64, t571: f64, t1401: f64, t574: f64, t1403: f64, t559: f64, t1356: f64, t593: f64, t1308: f64, t1446: f64, t1454: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3864 = t3863 * t1309;
    let t3865 = t571 * t3864;
    let t3866 = 16.0_f64 / 45.0_f64 * t3865;
    let t3867 = t574 * t1401;
    let t3868 = t559 * t1403;
    let t3869 = t3867 * t3868;
    let t3871 = 8.0_f64 / 15.0_f64 * t571 * t3869;
    let t3872 = t1356 * t593;
    let t3873 = t1308 * t3872;
    let t3875 = 8.0_f64 / 15.0_f64 * t571 * t3873;
    let t3877 = 4.0_f64 / 15.0_f64 * t1446 * t1454;
    (t3864, t3865, t3866, t3867, t3868, t3869, t3871, t3872, t3873, t3875, t3877)
}
