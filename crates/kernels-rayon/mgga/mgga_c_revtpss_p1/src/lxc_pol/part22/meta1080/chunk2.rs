//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3884/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3884(t124: f64, t22079: f64, t3924: f64, t3934: f64, t3944: f64, t47298: f64, t47304: f64, t49049: f64, t49053: f64, t49056: f64, t49058: f64, t49062: f64, t49066: f64, t49070: f64, t5673: f64, t73345: f64, t800: f64) -> f64 {
    let t74636 = -0.21437009059034868486e-3_f64 * t3934 * t5673 * t22079 * t3924 + t3944 * t800 * t124 * t73345 / 8.0_f64 + 0.2032800112371413129e-3_f64 * t49049 - 0.25410001404642664112e-4_f64 * t49053 + 0.4065600224742826258e-4_f64 * t49056 - 7.0_f64 / 12.0_f64 * t49058 - 0.40015750243531754508e-2_f64 * t49062 + 0.57165357490759649296e-4_f64 * t49066 - 0.18071592998981862716e-4_f64 * t49070 + 0.13552000749142754193e-3_f64 * t47298 - 0.56688979511669985553e-2_f64 * t47304;
    t74636
}
