//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1290/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1290(t15322: f64, t4414: f64, t12198: f64, t4002: f64, t13796: f64, t14637: f64, t3737: f64, t875: f64, t11354: f64, t11401: f64, t1185: f64, t12204: f64, t12237: f64, t13888: f64, t14403: f64, t14651: f64, t14791: f64, t15138: f64, t2408: f64, t27047: f64, t27105: f64, t3066: f64, t3067: f64, t3207: f64, t35566: f64, t53253: f64, t53374: f64, t53405: f64, t53407: f64, t53472: f64, t56199: f64, t8629: f64, t8654: f64, t8776: f64, t9283: f64, t938: f64) -> f64 {
    let t56385 = t4414 * t15322;
    let t56400 = t12198 * t4002;
    let t56404 = t14637 * t13796 * t3737 * t875;
    let t56425 = -t53374 - 7.0_f64 / 72.0_f64 * t56385 + t53405 - t8629 * t27047 * t3067 * t56199 * t938 / 48.0_f64 - t8629 * t53472 / 24.0_f64 + t8776 * t1185 * t15138 / 96.0_f64 + t8654 * t27105 * t14403 / 24.0_f64 + 7.0_f64 / 288.0_f64 * t56400 + t53407 - 5.0_f64 / 768.0_f64 * t56404 - t2408 * t9283 * t13888 * t11401 / 12.0_f64 - t3066 * t9283 * t14791 * t11354 / 16.0_f64 - t2408 * t35566 * t14651 / 12.0_f64 + t3066 * t9283 * t53253 * t12204 / 4.0_f64 + t3207 * t9283 * t13888 * t12237 / 8.0_f64;
    t56425
}
