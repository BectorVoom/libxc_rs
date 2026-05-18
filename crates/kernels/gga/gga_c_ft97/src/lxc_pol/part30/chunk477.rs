//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 477/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk477<F: Float>(t292: F, t1201: F, t5265: F, t7006: F, t7009: F, t7458: F, t7466: F, t7471: F, t7480: F, t7591: F, t7607: F) -> F {
    let t293 = F::new(0.1e-59) < t292;
    let t7611 = piecewise3::<f64>(t293, F::new(0.10263553471742804997e0) * t5265 * t7591 - F::new(0.41054213886971219988e0) * t1201 * t7466 - F::new(0.90629106640255751116e-1) * t7006 * t7471 + F::new(0.22653425206514361674e0) * t1201 * t7458 + F::new(0.20527106943485609994e0) * t292 * t7466 + F::new(0.90629106640255751116e-1) * t7009 * t7471 - F::new(0.22653425206514361674e0) * t292 * t7458 + F::new(0.40013602467334010748e-1) * t7607 * t7480, F::new(0.0));
    t7611
}
