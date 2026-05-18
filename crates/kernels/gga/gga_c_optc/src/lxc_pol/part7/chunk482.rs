//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 482/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk482<F: Float>(t2374: F, t799: F, t2373: F, t774: F, t778: F, t800: F, t214: F, t217: F, t782: F) -> (F, F, F, F, F, F) {
    let t2375 = t2374 * t799;
    let t2377 = F::new(2.0) * t2373 * t2375;
    let t2378 = t774 * t778;
    let t2380 = F::new(2.0) * t2378 * t800;
    let t2382 = F::new(1.0) / t217 / t214;
    let t2383 = t782 * t782;
    (t2375, t2377, t2378, t2380, t2382, t2383)
}
