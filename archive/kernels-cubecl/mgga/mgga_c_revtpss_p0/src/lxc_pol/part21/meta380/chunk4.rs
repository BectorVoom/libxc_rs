//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1796/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1796<F: Float>(t12430: F, t3479: F, t12252: F, t12259: F, t12261: F, t12263: F, t12265: F, t12271: F, t12275: F, t12279: F, t12284: F, t12289: F, t12292: F, t12323: F, t12329: F, t12332: F) -> (F, F) {
    let t12431 = t12430 * t3479;
    let t12448 = F::cast_from(0.69463333333333333335e-1_f64) * t12252 + F::cast_from(0.46308888888888888889e-1_f64) * t12259 + F::cast_from(0.34731666666666666667e0_f64) * t12261 - F::cast_from(0.20839e0_f64) * t12263 - F::cast_from(0.41678000000000000001e0_f64) * t12265 - F::cast_from(0.20839e0_f64) * t12271 + F::cast_from(0.62517e0_f64) * t12275 + F::cast_from(0.104195e0_f64) * t12279 - F::cast_from(0.104195e0_f64) * t12284 + F::cast_from(0.62517e0_f64) * t12289 - F::cast_from(0.103295e1_f64) * t12292 + F::cast_from(0.3529725e1_f64) * t12323 + F::cast_from(0.264729375e1_f64) * t12329 - F::cast_from(0.157790625e0_f64) * t12332;
    (t12431, t12448)
}
