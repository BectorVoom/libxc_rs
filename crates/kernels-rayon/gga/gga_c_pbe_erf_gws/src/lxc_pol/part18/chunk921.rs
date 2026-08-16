//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 921/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk921(t10295: f64, t661: f64, t1621: f64, t639: f64, t1630: f64, t3499: f64, t4993: f64, t3479: f64, t663: f64, t10290: f64, t10291: f64, t10294: f64, t4872: f64, t4873: f64, t4876: f64, t4910: f64, t6971: f64, t6998: f64, t7045: f64, t7047: f64, t7074: f64, t7075: f64) -> (f64, f64, f64, f64, f64) {
    let t10296 = t10295 * t661;
    let t10297 = t1621 * t10296;
    let t10299 = 4.0_f64 / 15.0_f64 * t639 * t10297;
    let t10300 = t1630 * t3499;
    let t10301 = t639 * t10300;
    let t10302 = 16.0_f64 / 135.0_f64 * t10301;
    let t10303 = 8.0_f64 / 405.0_f64 * t4993;
    let t10305 = 2.0_f64 / 15.0_f64 * t3479 * t663;
    let t10306 = -t4872 - t6971 + 4.0_f64 / 135.0_f64 * t6998 + 0.33245444444444444444e-1_f64 * t4873 + t4876 + t10290 + t4910 - t10291 - t7045 + t7047 + t7074 + 8.0_f64 / 9.0_f64 * t7075 + t10294 + t10299 - t10302 - t10303 - t10305;
    (t10299, t10302, t10303, t10305, t10306)
}
