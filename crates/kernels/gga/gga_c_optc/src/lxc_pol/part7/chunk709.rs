//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 709/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk709<F: Float>(t148: F, t151: F, t6568: F, t2168: F, t6778: F, t686: F, t6919: F, t6933: F, t6938: F, t705: F, t7074: F, t7076: F, t7078: F, t7083: F, t7086: F, t7091: F) -> (F,) {
    let t7094 = 0.29299173910028776472e1 * t148 * t6568 * t151;
    let t7101 = -0.40568086952347536654e1 * t7074 + 0.12170426085704260996e1 * t7076 - 0.2115989587251296286e1 * t7078 - 0.90685268025055555116e0 * t705 * t6919 - 0.20863587575493018851e1 * t686 * t7083 - 0.36511278257112782988e1 * t7086 - t7091 - t7094 - 0.90685268025055555117e0 * t2168 * t6933 + 0.18137053605011111023e0 * t2168 * t6938 - 0.45342634012527777558e-1 * t2168 * t6778;
    (t7101,)
}
