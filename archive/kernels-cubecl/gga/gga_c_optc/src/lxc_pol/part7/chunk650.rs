//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 650/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk650<F: Float>(t277: F, t2835: F, t2841: F, t2905: F, t2908: F, t2911: F, t2921: F, t2987: F, t3066: F, t3069: F, t3073: F, t3077: F, t3293: F, t498: F, t95: F) -> F {
    let t3294 = t2835 / F::cast_from(3.0_f64) - t2841 + t2905 * t498 / F::cast_from(2.0_f64) - F::cast_from(0.25844881434903430496e-2_f64) * t95 * t277 * t2908 * t2911 + t2921 + t3066 - t2987 - t3069 - t3073 - t3077 + t3293;
    t3294
}
