//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3274/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3274<F: Float>(t14923: F, t18456: F, t40850: F, t40851: F, t51070: F, t51074: F, t51078: F, t51081: F, t51083: F, t51086: F, t51089: F, t51092: F) -> F {
    let t62188 = t14923 * t18456;
    let t62199 = -F::cast_from(0.12004725073059526352e-1_f64) * t62188 + F::cast_from(0.57165357490759649296e-4_f64) * t51070 - F::cast_from(0.1219527626469539185e-2_f64) * t51074 - F::cast_from(0.18071592998981862716e-4_f64) * t51078 + F::cast_from(0.36143185997963725432e-4_f64) * t51081 + F::cast_from(0.90357964994909313586e-6_f64) * t51083 + F::cast_from(0.72286371995927450868e-4_f64) * t51086 + F::cast_from(0.72286371995927450867e-4_f64) * t51089 - F::cast_from(0.18071592998981862716e-4_f64) * t51092 - t40850 + F::cast_from(0.15244095330869239812e-2_f64) * t40851;
    t62199
}
