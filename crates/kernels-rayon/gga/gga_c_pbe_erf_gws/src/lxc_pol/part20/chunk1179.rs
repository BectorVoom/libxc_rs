//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1179/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1179(t1178: f64, t371: f64, t3887: f64, t1177: f64, t1118: f64, t1134: f64, t13796: f64, t13859: f64, t1115: f64, t14397: f64, t14400: f64, t14404: f64, t14420: f64, t14599: f64, t14898: f64, t15135: f64, t15139: f64, t15147: f64, t15152: f64, t15156: f64, t15162: f64, t15165: f64, t15170: f64, t3066: f64, t3917: f64, t4002: f64, t8629: f64, t8793: f64) -> (f64, f64, f64) {
    let t15177 = t371 * t1178 * t3887;
    let t15178 = t1177 * t15177;
    let t15181 = t1118 * t1134;
    let t15182 = t13796 * t15181;
    let t15183 = t13859 * t15182;
    let t15185 = 7.0_f64 / 144.0_f64 * t14400 + t8793 * t14420 / 24.0_f64 - t15135 / 768.0_f64 + t8629 * t15139 / 96.0_f64 + t8793 * t14404 / 24.0_f64 - t15147 / 768.0_f64 - t15152 / 1536.0_f64 + t3066 * t15156 / 48.0_f64 + t15162 / 96.0_f64 + t15165 / 48.0_f64 + t15170 / 1536.0_f64 - t3917 * t4002 / 96.0_f64 - t1115 * t14397 / 48.0_f64 - t15178 / 3072.0_f64 - 7.0_f64 / 72.0_f64 * t14599 + t15183 / 384.0_f64 + t14898;
    (t15177, t15182, t15185)
}
