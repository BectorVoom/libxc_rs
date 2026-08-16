//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 491/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk491(t1375: f64, t3575: f64, t1376: f64, t960: f64, t3583: f64, t457: f64, t1384: f64, t965: f64, t1383: f64, t1186: f64, t167: f64, t3532: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3870 = t1375 * t3575;
    let t3873 = t960 * t1376;
    let t3875 = t1375 * t3583;
    let t3878 = t457 * t3575;
    let t3881 = t965 * t1384;
    let t3883 = t1383 * t3583;
    let t3886 = t1186 * t3575;
    let t3891 = t167 * t3532;
    (t3870, t3873, t3875, t3878, t3881, t3883, t3886, t3891)
}
