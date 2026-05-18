//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 890/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk890<F: Float>(t9273: F, t9279: F, t9284: F, t9289: F, t9292: F, t9295: F, t9299: F, t9302: F, t9309: F, t9312: F, t9315: F, t9318: F, t9320: F) -> F {
    let t10767 = -F::new(0.91682472831214851819e-8) * t9273 - F::new(0.10129555677746642575e-5) * t9279 - F::new(0.49522272202316919253e-5) * t9284 + F::new(0.33765185592488808582e-6) * t9289 + F::new(0.67530371184977617164e-6) * t9292 - F::new(0.20241536458333333335e-4) * t9295 + F::new(0.10136107947527008247e-3) * t9299 + F::new(0.13900948042322754167e-3) * t9302 - F::new(0.33765185592488808582e-6) * t9309 - F::new(0.24761136101158459626e-5) * t9312 + F::new(0.34752370105806885418e-3) * t9315 + F::new(0.34752370105806885418e-3) * t9318 + F::new(0.2318836277704281739e-4) * t9320;
    t10767
}
