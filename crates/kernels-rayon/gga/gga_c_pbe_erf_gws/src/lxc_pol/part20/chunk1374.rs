//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1374/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1374(t2409: f64, t35890: f64, t3965: f64, t12243: f64, t14121: f64, t13772: f64, t3200: f64, t335: f64, t338: f64, t3917: f64, t4183: f64, t51957: f64, t54536: f64, t54538: f64, t54567: f64, t57581: f64, t57584: f64, t57588: f64, t57593: f64, t57595: f64, t57598: f64, t57602: f64, t57605: f64, t6793: f64) -> f64 {
    let t57608 = t3965 * t2409 * t35890;
    let t57614 = t14121 * t12243;
    let t57618 = -t54536 + t54538 - 7.0_f64 / 288.0_f64 * t57581 + t57584 / 768.0_f64 + t6793 * t57588 / 48.0_f64 + t57593 / 768.0_f64 + t57595 / 24.0_f64 - t57598 / 48.0_f64 + t54567 - t57602 / 384.0_f64 - t57605 / 48.0_f64 - t57608 / 96.0_f64 + t51957 - t335 * t338 * t3200 * t4183 / 48.0_f64 + t57614 / 16.0_f64 - t3917 * t13772 / 96.0_f64;
    t57618
}
