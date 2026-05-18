//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 83/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk83<F: Float>(t11: F, t14: F, t17: F, t25: F, t231: F, t33: F, t56: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t233 = F::new(1.0) + F::new(0.5137e-1) * t11;
    let t238 = F::new(0.705945e1) * t14 + F::new(0.1549425e1) * t11 + F::new(0.420775e0) * t17 + F::new(0.1562925e0) * t25;
    let t241 = F::new(1.0) + F::new(0.32164683177870697974e2) / t238;
    let t242 = f64::ln(t241);
    let t247 = t231 * (-F::new(0.3109e-1) * t233 * t242 + t33 - F::new(0.19751789702565206229e-1) * t56);
    let t249 = F::new(0.19751789702565206229e-1) * t231 * t56;
    let t252 = F::new(0.149676e1) + F::new(0.89527e-3) * t14 + F::new(0.11799625e-1) * t11;
    let t255 = F::new(1.0) + t14 * t252 / F::new(2.0);
    let t256 = t255 * t255;
    let t257 = F::new(1.0) / t256;
    (t233, t238, t241, t242, t247, t249, t252, t255, t256, t257)
}
