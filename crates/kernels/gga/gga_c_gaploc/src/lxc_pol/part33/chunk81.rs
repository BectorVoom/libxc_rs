//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 81/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk81<F: Float>(t11: F, t14: F, t17: F, t25: F, t231: F, t33: F, t56: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t233 = 1.0 + 0.5137e-1 * t11;
    let t238 = 0.705945e1 * t14 + 0.1549425e1 * t11 + 0.420775e0 * t17 + 0.1562925e0 * t25;
    let t241 = 1.0 + 0.32164683177870697974e2 / t238;
    let t242 = f64::ln(t241);
    let t247 = t231 * (-0.3109e-1 * t233 * t242 + t33 - 0.19751789702565206229e-1 * t56);
    let t249 = 0.19751789702565206229e-1 * t231 * t56;
    let t252 = 0.149676e1 + 0.89527e-3 * t14 + 0.11799625e-1 * t11;
    let t255 = 1.0 + t14 * t252 / 2.0;
    let t256 = t255 * t255;
    let t257 = 1.0 / t256;
    (t233, t238, t241, t242, t247, t249, t252, t255, t256, t257)
}
