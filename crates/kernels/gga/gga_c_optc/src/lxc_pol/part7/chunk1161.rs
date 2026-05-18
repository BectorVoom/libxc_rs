//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1161/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1161<F: Float>(t2264: F, t24128: F, t7222: F, t7218: F, t7307: F, t2441: F, t7318: F, t2529: F, t7604: F, t838: F, t845: F, t23: F, t2326: F, t2328: F, t2331: F) -> (F, F, F, F, F) {
    let t24130 = t24128 * t7222 * t2264;
    let t24133 = t7307 * t7218;
    let t24137 = F::new(0.2077890707925103596e3) * t2441 * t7318;
    let t24141 = F::new(0.46785787179641632568e1) * t845 * t2529 * t7604 * t838;
    let t24145 = t2326 * t2328 * t2331 * t23;
    (t24130, t24133, t24137, t24141, t24145)
}
