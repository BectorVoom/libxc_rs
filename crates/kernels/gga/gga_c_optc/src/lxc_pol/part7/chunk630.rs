//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 630/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk630<F: Float>(t3118: F, t3120: F, t241: F, t3030: F, t2921: F, t2987: F, t2990: F, t2997: F, t3015: F, t3023: F, t3066: F, t3069: F, t3073: F, t3077: F) -> (F, F, F) {
    let t3121 = t3118 * t3120;
    let t3125 = F::new(0.19751789702565206229e-1) * t241 * t3030;
    let t3126 = -t2987 + t2990 - t2997 + t3015 + t3023 + t3066 + t3125 - t3069 + t2921 - t3073 - t3077;
    (t3121, t3125, t3126)
}
