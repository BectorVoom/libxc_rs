//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1376/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1376(t15191: f64, t50994: f64, t1113: f64, t13781: f64, t3306: f64, t3972: f64, t824: f64, t11348: f64, t4002: f64, t13808: f64, t15151: f64, t12182: f64, t13792: f64) -> (f64, f64, f64, f64, f64) {
    let t57643 = t50994 * t15191;
    let t57648 = t3972 * t13781 * t1113 * t824 * t3306;
    let t57650 = t11348 * t4002;
    let t57652 = t13808 * t15151;
    let t57654 = t13792 * t12182;
    (t57643, t57648, t57650, t57652, t57654)
}
