//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1263/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1263(t13781: f64, t3222: f64, t3306: f64, t3972: f64, t14657: f64, t50891: f64, t1114: f64, t51916: f64, t51919: f64, t13888: f64, t2408: f64, t51505: f64, t51507: f64, t53526: f64, t53529: f64, t53531: f64, t53537: f64, t53542: f64, t53544: f64, t53546: f64, t53549: f64, t53553: f64, t8764: f64, t9283: f64, param_a_c: f64) -> f64 {
    let t53562 = t3972 * t13781 * t3306 * param_a_c * t3222;
    let t53564 = t14657 * t50891;
    let t53566 = t1114 * t51916;
    let t53567 = t53566 * t51919;
    let t53569 = 5.0_f64 / 384.0_f64 * t53526 + t53529 / 768.0_f64 + t53531 / 24.0_f64 - 7.0_f64 / 288.0_f64 * t51505 - 7.0_f64 / 2304.0_f64 * t51507 - t53537 / 3072.0_f64 + t53542 / 1536.0_f64 - t53544 - t53546 - 5.0_f64 / 768.0_f64 * t53549 + t53553 / 768.0_f64 - t2408 * t9283 * t13888 * t8764 / 24.0_f64 - t53562 / 768.0_f64 - t53564 / 48.0_f64 + t53567 / 48.0_f64;
    t53569
}
