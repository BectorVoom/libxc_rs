//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3884/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3884<F: Float>(t124: F, t22079: F, t3924: F, t3934: F, t3944: F, t47298: F, t47304: F, t49049: F, t49053: F, t49056: F, t49058: F, t49062: F, t49066: F, t49070: F, t5673: F, t73345: F, t800: F) -> F {
    let t74636 = -F::cast_from(0.21437009059034868486e-3_f64) * t3934 * t5673 * t22079 * t3924 + t3944 * t800 * t124 * t73345 / F::new(8.0) + F::cast_from(0.2032800112371413129e-3_f64) * t49049 - F::cast_from(0.25410001404642664112e-4_f64) * t49053 + F::cast_from(0.4065600224742826258e-4_f64) * t49056 - F::new(7.0) / F::new(12.0) * t49058 - F::cast_from(0.40015750243531754508e-2_f64) * t49062 + F::cast_from(0.57165357490759649296e-4_f64) * t49066 - F::cast_from(0.18071592998981862716e-4_f64) * t49070 + F::cast_from(0.13552000749142754193e-3_f64) * t47298 - F::cast_from(0.56688979511669985553e-2_f64) * t47304;
    t74636
}
