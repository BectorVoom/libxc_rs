//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 934/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk934<F: Float>(t9273: F, t9279: F, t9284: F, t9289: F, t9292: F, t9295: F, t9299: F, t9302: F, t9309: F, t9312: F, t9315: F, t9318: F, t9320: F) -> F {
    let t10767 = -F::cast_from(0.91682472831214851819e-8_f64) * t9273 - F::cast_from(0.10129555677746642575e-5_f64) * t9279 - F::cast_from(0.49522272202316919253e-5_f64) * t9284 + F::cast_from(0.33765185592488808582e-6_f64) * t9289 + F::cast_from(0.67530371184977617164e-6_f64) * t9292 - F::cast_from(0.20241536458333333335e-4_f64) * t9295 + F::cast_from(0.10136107947527008247e-3_f64) * t9299 + F::cast_from(0.13900948042322754167e-3_f64) * t9302 - F::cast_from(0.33765185592488808582e-6_f64) * t9309 - F::cast_from(0.24761136101158459626e-5_f64) * t9312 + F::cast_from(0.34752370105806885418e-3_f64) * t9315 + F::cast_from(0.34752370105806885418e-3_f64) * t9318 + F::cast_from(0.2318836277704281739e-4_f64) * t9320;
    t10767
}
