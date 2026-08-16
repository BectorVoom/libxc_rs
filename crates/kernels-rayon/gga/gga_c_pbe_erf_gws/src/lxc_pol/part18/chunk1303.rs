//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1303/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1303(t13781: f64, t15144: f64, t3038: f64, t3972: f64, t1115: f64, t12255: f64, t13939: f64, t14437: f64, t14791: f64, t2408: f64, t2498: f64, t3040: f64, t3913: f64, t4002: f64, t52897: f64, t53681: f64, t56604: f64, t56613: f64, t56618: f64, t56620: f64, t56626: f64, t56638: f64, t56642: f64, t56647: f64, t9283: f64, t9958: f64) -> f64 {
    let t56651 = t3972 * t13781 * t3038 * t15144;
    let t56653 = t56604 / 384.0_f64 + t2408 * t9283 * t14791 * t12255 / 8.0_f64 - t56613 / 1536.0_f64 + t56618 / 768.0_f64 + 7.0_f64 / 144.0_f64 * t56620 - t3913 * t13939 / 96.0_f64 - t1115 * t52897 / 48.0_f64 - t56626 / 96.0_f64 - t9958 * t4002 / 96.0_f64 - t3040 * t14437 / 48.0_f64 - t2498 * t14437 / 48.0_f64 - t1115 * t53681 / 48.0_f64 - t56638 / 768.0_f64 - t56642 / 1536.0_f64 + t56647 / 384.0_f64 - t56651 / 768.0_f64;
    t56653
}
