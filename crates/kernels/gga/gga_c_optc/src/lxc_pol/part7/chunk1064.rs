//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1064/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1064<F: Float>(t2432: F, t7182: F, t1007: F, t1008: F, t6: F, t7312: F, t2274: F, t7221: F, t7222: F, t190: F, t2548: F, t136: F, t2264: F, t7218: F, t7307: F, t2441: F, t7318: F) -> (F, F, F, F, F, F) {
    let t24109 = t7182 * t2432;
    let t24120 = t1007 * t1008 * t7312 * t6;
    let t24124 = t7221 * t7222 * t2274;
    let t24127 = t2548 * t190;
    let t24128 = t24127 * t136;
    let t24130 = t24128 * t7222 * t2264;
    let t24133 = t7307 * t7218;
    let t24137 = 0.2077890707925103596e3 * t2441 * t7318;
    (t24109, t24120, t24124, t24130, t24133, t24137)
}
