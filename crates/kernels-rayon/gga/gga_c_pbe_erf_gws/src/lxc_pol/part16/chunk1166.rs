//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1166/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1166(t15081: f64, t898: f64, t338: f64, t353: f64, t2409: f64, t4088: f64, t8589: f64, t14129: f64, t14131: f64, t14182: f64, t14193: f64, t14800: f64, t14806: f64, t14812: f64, t15018: f64, t15022: f64, t15027: f64, t15036: f64, t2408: f64, t3066: f64, t335: f64, t6793: f64, t8629: f64, t8793: f64) -> (f64, f64, f64, f64) {
    let t15082 = t898 * t15081;
    let t15084 = t338 * t353 * t15082;
    let t15089 = t2409 * t8589 * t4088;
    let t15094 = -t14129 - t335 * t15018 / 96.0_f64 - t2408 * t15022 / 24.0_f64 + t3066 * t15027 / 48.0_f64 + t8793 * t14182 / 48.0_f64 + t8629 * t14193 / 96.0_f64 + t6793 * t15036 / 48.0_f64 - t335 * t15084 / 96.0_f64 - t14131 + t14800 / 768.0_f64 + t2408 * t15089 / 48.0_f64 + t14806 / 24.0_f64 + t14812 / 24.0_f64;
    (t15082, t15084, t15089, t15094)
}
