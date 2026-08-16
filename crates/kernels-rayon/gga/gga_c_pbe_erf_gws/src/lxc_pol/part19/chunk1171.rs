//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1171/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1171(t14558: f64, t14563: f64, t14060: f64, t14081: f64, t14229: f64, t14233: f64, t14556: f64, t14560: f64, t14568: f64, t14571: f64, t15072: f64, t15049: f64, t15060: f64, t15071: f64) -> f64 {
    let t15074 = 7.0_f64 / 288.0_f64 * t14558;
    let t15076 = 7.0_f64 / 72.0_f64 * t14563;
    let t15079 = t15072 - t14556 / 192.0_f64 + t15074 - t14560 / 96.0_f64 + t14060 + t15076 + t14568 / 48.0_f64 - t14571 / 48.0_f64 + t14229 + t14081 + t14233;
    let t15081 = t15049 + t15060 + t15071 + t15079;
    t15081
}
