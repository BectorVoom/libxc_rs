//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1276/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1276(t1134: f64, t13917: f64, t53799: f64, t824: f64, t938: f64, t15156: f64, t9270: f64, t15351: f64, t22534: f64, t2409: f64, t3066: f64, t3068: f64, t4182: f64, t50949: f64, t52962: f64, t52969: f64, t52971: f64, t52973: f64, t52992: f64, t56166: f64, t56168: f64, t56170: f64, t56174: f64, t56176: f64, t6126: f64, t9283: f64) -> f64 {
    let t56181 = t13917 * t53799 * t824 * t1134 * t938;
    let t56183 = t9270 * t15156;
    let t56189 = -t3066 * t9283 * t6126 * t4182 * t3068 / 8.0_f64 - t52962 + t52969 + t52971 + t52973 + 119.0_f64 / 6912.0_f64 * t50949 - t56166 / 1536.0_f64 - t56168 / 24.0_f64 + t56170 / 8.0_f64 - t56174 / 1536.0_f64 + t56176 / 24.0_f64 + t56181 / 768.0_f64 - 7.0_f64 / 144.0_f64 * t56183 - t3066 * t2409 * t22534 * t15351 / 16.0_f64 + t52992;
    t56189
}
