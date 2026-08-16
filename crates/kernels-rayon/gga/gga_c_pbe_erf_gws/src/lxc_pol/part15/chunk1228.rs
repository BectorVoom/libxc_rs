//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1228/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1228(t13772: f64, t14397: f64, t14437: f64, t14791: f64, t2388: f64, t2392: f64, t2408: f64, t3040: f64, t50927: f64, t52940: f64, t52944: f64, t52952: f64, t52956: f64, t52959: f64, t52962: f64, t52969: f64, t52971: f64, t52973: f64, t52976: f64, t9218: f64, t9283: f64) -> f64 {
    let t52978 = -t2388 * t14437 / 96.0_f64 + t52940 / 384.0_f64 + t52944 / 768.0_f64 + t2408 * t9283 * t14791 * t9218 / 8.0_f64 - t52952 / 3072.0_f64 + t52956 / 768.0_f64 - t52959 / 192.0_f64 - t52962 + 7.0_f64 / 1152.0_f64 * t50927 - t3040 * t13772 / 48.0_f64 - t2392 * t14397 / 96.0_f64 + t52969 + t52971 + t52973 + t52976 / 768.0_f64;
    t52978
}
