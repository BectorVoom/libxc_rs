//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1026/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1026<F: Float>(t3475: F, t431: F, t426: F, t1168: F, t3453: F, t3479: F, t12252: F, t12259: F, t12261: F, t12263: F, t12265: F, t12271: F, t12275: F, t12279: F, t12284: F, t12289: F, t12292: F, t12323: F, t12329: F, t12332: F) -> (F, F, F, F) {
    let t12428 = F::new(1.0) / t3475 / t431;
    let t12429 = t426 * t12428;
    let t12430 = t3453 * t1168;
    let t12431 = t12430 * t3479;
    let t12448 = F::cast_from(0.69463333333333333335e-1_f64) * t12252 + F::cast_from(0.46308888888888888889e-1_f64) * t12259 + F::cast_from(0.34731666666666666667e0_f64) * t12261 - F::new(0.20839e0) * t12263 - F::cast_from(0.41678000000000000001e0_f64) * t12265 - F::new(0.20839e0) * t12271 + F::new(0.62517e0) * t12275 + F::new(0.104195e0) * t12279 - F::new(0.104195e0) * t12284 + F::new(0.62517e0) * t12289 - F::new(0.103295e1) * t12292 + F::new(0.3529725e1) * t12323 + F::cast_from(0.264729375e1_f64) * t12329 - F::cast_from(0.157790625e0_f64) * t12332;
    (t12429, t12430, t12431, t12448)
}
