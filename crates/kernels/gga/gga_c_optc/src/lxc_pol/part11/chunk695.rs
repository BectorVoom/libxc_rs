//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 695/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk695<F: Float>(t146: F, t2111: F, t622: F, t155: F, t158: F, t2078: F, t147: F, t2002: F, t136: F, t162: F, t6165: F, t159: F) -> (F, F, F, F) {
    let t7037 = t146 * t2111 * t622;
    let t7061 = t155 * t158 * t2078;
    let t7073 = t146 * t147 * t2002;
    let t7089 = t6165 * t136 * t162;
    let t7091 = F::new(0.13322897401211865505e1) * t159 * t7089;
    (t7037, t7061, t7073, t7091)
}
